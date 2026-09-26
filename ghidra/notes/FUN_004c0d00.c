
void __fastcall FUN_004c0d00(int param_1)

{
  uint uVar1;
  
  if (*(int *)(param_1 + 0x10) == 0) {
    if ((DAT_006b28b0 & 0x8000000) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x6c) = DAT_006b28cc + 0x32;
      DAT_006b28b0 = DAT_006b28b0 & 0xf7ffffff;
      *(undefined4 *)(param_1 + 0x10) = 1;
    }
    DAT_006b28b0 = DAT_006b28b0 & 0xc000;
  }
  else {
    DAT_006b28bc = 0;
    if ((DAT_006b28b0 & 0x4000000) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x68) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xfbffffff;
      *(undefined4 *)(param_1 + 0x10) = 0;
      FUN_0043a230(param_1);
      return;
    }
    if ((DAT_006b28b0 & 0x2000) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x124) = DAT_006b28cc + 40000;
      FUN_0041da30();
      DAT_006b28b0 = DAT_006b28b0 & 0xffffdfff;
      return;
    }
    if ((DAT_006b28b0 & 1) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x70) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xfffffffe;
      return;
    }
    if ((DAT_006b28b0 & 0x10) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x74) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xffffffef;
      return;
    }
    if ((DAT_006b28b0 & 0x20) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x7c) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xffffffdf;
      return;
    }
    if ((DAT_006b28b0 & 0x100) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x78) = DAT_006b28cc + 0x32;
      DAT_006b28b0 = DAT_006b28b0 & 0xfffffeff;
      return;
    }
    if ((DAT_006b28b0 & 0x200) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x80) = DAT_006b28cc + 0x32;
      DAT_006b28b0 = DAT_006b28b0 & 0xfffffdff;
      return;
    }
    if ((DAT_006b28b0 & 0x2000000) != 0) {
      uVar1 = DAT_006b28b8 + 0x49;
      if (uVar1 < 0x4b) {
        uVar1 = 0x4b;
      }
      else if (0x4d < uVar1) {
        uVar1 = 0x4d;
        DAT_006b28b8 = 0;
      }
      *(int *)(*(int *)(param_1 + 0x168) + uVar1 * 4) = DAT_006b28cc + 5;
      DAT_006b28b0 = DAT_006b28b0 & 0xfdffffff;
      return;
    }
    if ((DAT_006b28b0 & 0x80000000) != 0) {
      uVar1 = FUN_0041cd80(0xc);
      if (uVar1 < 6) {
        *(int *)(*(int *)(param_1 + 0x168) + 0x138 + uVar1 * 4) = DAT_006b28cc + 5;
      }
      DAT_006b28bc = 1;
      DAT_006b28b0 = DAT_006b28b0 & 0x7fffffff;
      return;
    }
  }
  return;
}

