
bool __fastcall FUN_0059f060(int param_1)

{
  undefined4 *puVar1;
  int iVar2;
  uint local_68 [26];
  
  *(undefined4 *)(param_1 + 0xc) = *(undefined4 *)(param_1 + 0x74);
  *(undefined4 *)(param_1 + 0x10) = *(undefined4 *)(param_1 + 0x78);
  *(undefined4 *)(param_1 + 0x14) = *(undefined4 *)(param_1 + 0x7c);
  *(undefined4 *)(param_1 + 0x18) = *(undefined4 *)(param_1 + 0x80);
  *(undefined4 *)(param_1 + 0x1c) = *(undefined4 *)(param_1 + 0x84);
  *(undefined4 *)(param_1 + 0x20) = *(undefined4 *)(param_1 + 0x88);
  *(undefined4 *)(param_1 + 0x24) = *(undefined4 *)(param_1 + 0x8c);
  *(undefined4 *)(param_1 + 0x28) = *(undefined4 *)(param_1 + 0x90);
  *(undefined4 *)(param_1 + 0x2c) = *(undefined4 *)(param_1 + 0x94);
  *(undefined4 *)(param_1 + 0x30) = *(undefined4 *)(param_1 + 0x98);
  *(undefined4 *)(param_1 + 0x34) = *(undefined4 *)(param_1 + 0x9c);
  *(undefined4 *)(param_1 + 0x38) = *(undefined4 *)(param_1 + 0xa0);
  *(undefined4 *)(param_1 + 0x3c) = *(undefined4 *)(param_1 + 0xa4);
  *(undefined4 *)(param_1 + 0x40) = *(undefined4 *)(param_1 + 0xa8);
  *(undefined4 *)(param_1 + 0x44) = *(undefined4 *)(param_1 + 0xac);
  *(undefined4 *)(param_1 + 0x48) = *(undefined4 *)(param_1 + 0xb0);
  *(undefined4 *)(param_1 + 0x4c) = *(undefined4 *)(param_1 + 0xb4);
  *(undefined4 *)(param_1 + 0x50) = *(undefined4 *)(param_1 + 0xb8);
  *(undefined4 *)(param_1 + 0x54) = *(undefined4 *)(param_1 + 0xbc);
  *(undefined4 *)(param_1 + 0x58) = *(undefined4 *)(param_1 + 0xc0);
  *(undefined4 *)(param_1 + 0x5c) = *(undefined4 *)(param_1 + 0xc4);
  *(undefined4 *)(param_1 + 0x60) = *(undefined4 *)(param_1 + 200);
  *(undefined4 *)(param_1 + 100) = *(undefined4 *)(param_1 + 0xcc);
  *(undefined4 *)(param_1 + 0x68) = *(undefined4 *)(param_1 + 0xd0);
  *(undefined4 *)(param_1 + 0x6c) = *(undefined4 *)(param_1 + 0xd4);
  *(undefined4 *)(param_1 + 0x70) = *(undefined4 *)(param_1 + 0xd8);
  if (*(void **)(param_1 + 4) != (void *)0x0) {
    puVar1 = (undefined4 *)FUN_005b1c00(*(void **)(param_1 + 4),local_68);
    *(undefined4 *)(param_1 + 0x74) = *puVar1;
    *(undefined4 *)(param_1 + 0x78) = puVar1[1];
    *(undefined4 *)(param_1 + 0x7c) = puVar1[2];
    *(undefined4 *)(param_1 + 0x80) = puVar1[3];
    *(undefined4 *)(param_1 + 0x84) = puVar1[4];
    *(undefined4 *)(param_1 + 0x88) = puVar1[5];
    *(undefined4 *)(param_1 + 0x8c) = puVar1[6];
    *(undefined4 *)(param_1 + 0x90) = puVar1[7];
    *(undefined4 *)(param_1 + 0x94) = puVar1[8];
    *(undefined4 *)(param_1 + 0x98) = puVar1[9];
    *(undefined4 *)(param_1 + 0x9c) = puVar1[10];
    *(undefined4 *)(param_1 + 0xa0) = puVar1[0xb];
    *(undefined4 *)(param_1 + 0xa4) = puVar1[0xc];
    *(undefined4 *)(param_1 + 0xa8) = puVar1[0xd];
    *(undefined4 *)(param_1 + 0xac) = puVar1[0xe];
    *(undefined4 *)(param_1 + 0xb0) = puVar1[0xf];
    *(undefined4 *)(param_1 + 0xb4) = puVar1[0x10];
    *(undefined4 *)(param_1 + 0xb8) = puVar1[0x11];
    *(undefined4 *)(param_1 + 0xbc) = puVar1[0x12];
    *(undefined4 *)(param_1 + 0xc0) = puVar1[0x13];
    *(undefined4 *)(param_1 + 0xc4) = puVar1[0x14];
    *(undefined4 *)(param_1 + 200) = puVar1[0x15];
    *(undefined4 *)(param_1 + 0xcc) = puVar1[0x16];
    *(undefined4 *)(param_1 + 0xd0) = puVar1[0x17];
    *(undefined4 *)(param_1 + 0xd4) = puVar1[0x18];
    *(undefined4 *)(param_1 + 0xd8) = puVar1[0x19];
    iVar2 = FUN_0059f1e0((undefined4 *)(param_1 + 0x74),param_1 + 0xc);
    return (bool)('\x01' - (iVar2 != 0));
  }
  return false;
}

