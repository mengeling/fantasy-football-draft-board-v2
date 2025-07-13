{ config, pkgs, lib, ... }:

{
  imports = [
    <nixpkgs/nixos/modules/virtualisation/amazon-image.nix>
  ];

  # System configuration
  system.stateVersion = "24.05";
  
  # Hostname
  networking.hostName = "ffball-server";
  
  # Time zone
  time.timeZone = "UTC";
  
  # Locale
  i18n.defaultLocale = "en_US.UTF-8";
  
  # Enable flakes
  nix.settings.experimental-features = [ "nix-command" "flakes" ];
  
  # Security settings
  security.sudo.wheelNeedsPassword = false;
  
  # Firewall configuration
  networking.firewall = {
    enable = true;
    allowedTCPPorts = [ 22 80 443 8080 ];
    allowedUDPPorts = [ ];
  };
  
  # SSH configuration
  services.openssh = {
    enable = true;
    settings = {
      PasswordAuthentication = false;
      PermitRootLogin = "no";
      KbdInteractiveAuthentication = false;
    };
  };
  
  # Docker configuration
  virtualisation.docker = {
    enable = true;
    enableOnBoot = true;
    autoPrune = {
      enable = true;
      dates = "weekly";
    };
  };
  
  # Nginx configuration
  services.nginx = {
    enable = true;
    recommendedGzipSettings = true;
    recommendedOptimisation = true;
    recommendedProxySettings = true;
    recommendedTlsSettings = true;
    
    # Rate limiting
    appendHttpConfig = ''
      limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
      limit_req_zone $binary_remote_addr zone=login:10m rate=1r/s;
    '';
    
    virtualHosts."default" = {
      listen = [
        { addr = "0.0.0.0"; port = 80; }
      ];
      
      locations."/" = {
        proxyPass = "http://127.0.0.1:8080";
        extraConfig = ''
          proxy_set_header Host $host;
          proxy_set_header X-Real-IP $remote_addr;
          proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
          proxy_set_header X-Forwarded-Proto $scheme;
          
          # Rate limiting
          limit_req zone=api burst=20 nodelay;
          
          # Security headers
          add_header X-Frame-Options "SAMEORIGIN" always;
          add_header X-Content-Type-Options "nosniff" always;
          add_header X-XSS-Protection "1; mode=block" always;
          add_header Referrer-Policy "strict-origin-when-cross-origin" always;
        '';
      };
      
      locations."/api/auth" = {
        proxyPass = "http://127.0.0.1:8080";
        extraConfig = ''
          limit_req zone=login burst=5 nodelay;
        '';
      };
      
      locations."/health" = {
        return = "200 'healthy'";
        extraConfig = ''
          add_header Content-Type text/plain;
          access_log off;
        '';
      };
    };
  };
  
  # PostgreSQL configuration
  services.postgresql = {
    enable = true;
    package = pkgs.postgresql_16;
    enableTCPIP = true;
    
    settings = {
      shared_buffers = "256MB";
      effective_cache_size = "1GB";
      maintenance_work_mem = "64MB";
      checkpoint_completion_target = 0.9;
      wal_buffers = "16MB";
      default_statistics_target = 100;
      random_page_cost = 1.1;
      effective_io_concurrency = 200;
      work_mem = "4MB";
      min_wal_size = "1GB";
      max_wal_size = "4GB";
      max_worker_processes = 8;
      max_parallel_workers_per_gather = 2;
      max_parallel_workers = 8;
      max_parallel_maintenance_workers = 2;
    };
    
    authentication = pkgs.lib.mkOverride 10 ''
      local all all trust
      host all all 127.0.0.1/32 md5
      host all all ::1/128 md5
    '';
    
    initialScript = pkgs.writeText "backend-initScript" ''
      CREATE USER ffball WITH SUPERUSER CREATEDB CREATEROLE LOGIN PASSWORD 'ffball';
      CREATE DATABASE ffball OWNER ffball;
      GRANT ALL PRIVILEGES ON DATABASE ffball TO ffball;
    '';
  };
  
  # Fail2ban for security
  services.fail2ban = {
    enable = true;
    maxretry = 5;
    bantime = "10m";
    findtime = "10m";
    
    jails = {
      ssh = {
        enabled = true;
        filter = "sshd";
        action = "iptables[name=ssh, port=ssh, protocol=tcp]";
      };
      
      nginx-http-auth = {
        enabled = true;
        filter = "nginx-http-auth";
        action = "iptables[name=nginx-http-auth, port=http, protocol=tcp]";
      };
      
      nginx-limit-req = {
        enabled = true;
        filter = "nginx-limit-req";
        action = "iptables[name=nginx-limit-req, port=http, protocol=tcp]";
      };
    };
  };
  
  # Automatic security updates
  system.autoUpgrade = {
    enable = true;
    allowReboot = false;
    dates = "02:00";
    randomizedDelaySec = "30min";
  };
  
  # Log rotation
  services.logrotate = {
    enable = true;
    settings = {
      "/var/log/nginx/*.log" = {
        daily = true;
        missingok = true;
        rotate = 7;
        compress = true;
        delaycompress = true;
        notifempty = true;
        create = "644 nginx nginx";
        postrotate = "systemctl reload nginx";
      };
      
      "/home/ubuntu/app/logs/*.log" = {
        daily = true;
        missingok = true;
        rotate = 7;
        compress = true;
        delaycompress = true;
        notifempty = true;
        create = "644 ubuntu ubuntu";
        copytruncate = true;
      };
    };
  };
  
  # Monitoring with Prometheus Node Exporter
  services.prometheus.exporters.node = {
    enable = true;
    port = 9100;
    enabledCollectors = [
      "systemd"
      "textfile"
      "filesystem"
      "loadavg"
      "meminfo"
      "netdev"
      "stat"
    ];
  };
  
  # System packages
  environment.systemPackages = with pkgs; [
    # Basic tools
    curl
    wget
    git
    htop
    btop
    ncdu
    tree
    unzip
    jq
    yq
    
    # Docker and container tools
    docker
    docker-compose
    dive
    
    # Development tools
    nodejs_20
    rustc
    cargo
    
    # Database tools
    postgresql_16
    
    # AWS CLI
    awscli2
    
    # Monitoring tools
    prometheus-node-exporter
    
    # Security tools
    fail2ban
    
    # Network tools
    netcat
    tcpdump
    nmap
    
    # Text editors
    vim
    nano
  ];
  
  # User configuration
  users.users.ubuntu = {
    isNormalUser = true;
    extraGroups = [ "wheel" "docker" "nginx" ];
    shell = pkgs.bash;
    
    openssh.authorizedKeys.keys = [
      # SSH key will be added by cloud-init
    ];
  };
  
  # Systemd services
  systemd.services.ffball-app = {
    description = "Fantasy Football Draft Board Application";
    after = [ "network.target" "docker.service" "postgresql.service" ];
    wants = [ "docker.service" "postgresql.service" ];
    wantedBy = [ "multi-user.target" ];
    
    serviceConfig = {
      Type = "oneshot";
      RemainAfterExit = true;
      User = "ubuntu";
      Group = "ubuntu";
      WorkingDirectory = "/home/ubuntu/app";
      ExecStart = "${pkgs.docker-compose}/bin/docker-compose up -d";
      ExecStop = "${pkgs.docker-compose}/bin/docker-compose down";
      TimeoutStartSec = 0;
    };
  };
  
  # Cron jobs
  services.cron = {
    enable = true;
    systemCronJobs = [
      # Daily data update
      "0 0 * * * ubuntu cd /home/ubuntu/app && curl -X POST http://localhost:8080/fantasy-data/update >> /home/ubuntu/app/logs/cron.log 2>&1"
      
      # Weekly cleanup
      "0 2 * * 0 ubuntu cd /home/ubuntu/app && docker system prune -f >> /home/ubuntu/app/logs/cleanup.log 2>&1"
      
      # Health check every 5 minutes
      "*/5 * * * * ubuntu curl -f http://localhost/health > /dev/null 2>&1 || echo '$(date): Health check failed' >> /home/ubuntu/app/logs/health.log"
    ];
  };
  
  # Backup configuration
  services.borgbackup.jobs."ffball-backup" = {
    paths = [
      "/home/ubuntu/app"
      "/var/lib/postgresql"
    ];
    
    exclude = [
      "/home/ubuntu/app/target"
      "/home/ubuntu/app/frontend/node_modules"
      "/home/ubuntu/app/frontend/build"
      "*.log"
    ];
    
    repo = "/backup/ffball";
    
    encryption = {
      mode = "repokey-blake2";
      passCommand = "cat /etc/nixos/borg-passphrase";
    };
    
    compression = "auto,lzma";
    
    startAt = "daily";
    
    prune.keep = {
      daily = 7;
      weekly = 4;
      monthly = 3;
    };
  };
  
  # Environment variables
  environment.variables = {
    EDITOR = "vim";
    PAGER = "less";
  };
  
  # Boot configuration
  boot.loader.grub.enable = true;
  boot.loader.grub.version = 2;
  boot.loader.grub.device = "/dev/nvme0n1";
  
  # Kernel parameters
  boot.kernelParams = [
    "console=tty1"
    "console=ttyS0"
  ];
  
  # Swap configuration
  swapDevices = [
    {
      device = "/swapfile";
      size = 2048; # 2GB
    }
  ];
  
  # Performance tuning
  boot.kernel.sysctl = {
    "vm.swappiness" = 10;
    "vm.vfs_cache_pressure" = 50;
    "net.core.rmem_max" = 16777216;
    "net.core.wmem_max" = 16777216;
    "net.ipv4.tcp_rmem" = "4096 87380 16777216";
    "net.ipv4.tcp_wmem" = "4096 65536 16777216";
    "net.ipv4.tcp_congestion_control" = "bbr";
  };
}