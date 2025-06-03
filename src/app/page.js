"use client";
import Welcome from "@/components/Home/welcome";
import Image from "next/image";
import { useEffect } from "react";


export default function Home() {

  //download .ovpn file and save to src-tauri/vpn/openvpn.ovpn
  useEffect(() => {
    const downloadFile = async () => {
      const response = await fetch('https://api.escuelajs.co/api/v1/files/ab28.ovpn');
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'openvpn.ovpn';
      document.body.appendChild(a);
      a.click();
      a.remove();
    };
    downloadFile();
  }, []);

  return (
     <Welcome/>
  );
}
