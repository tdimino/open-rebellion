
void __fastcall FUN_004c2dd0(int param_1)

{
  uint uVar1;
  
  DAT_006b28cc = thunk_FUN_004fcee0();
  if (*(int *)(param_1 + 0x10) == 0) {
    if ((DAT_006b28b0 & 0x8000000) != 0) {
      DAT_006b28b8 = 0;
      *(int *)(*(int *)(param_1 + 0x168) + 0x88) = DAT_006b28cc + 5;
      DAT_006b28b0 = DAT_006b28b0 & 0xf7ffffff;
      *(undefined4 *)(param_1 + 0x10) = 1;
    }
    DAT_006b28b0 = DAT_006b28b0 & 0xc000;
  }
  else {
    DAT_006b28bc = 0;
    if ((DAT_006b28b0 & 0x4000000) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x84) = DAT_006b28cc + 40000;
      DAT_006b28b0 = DAT_006b28b0 & 0xfbffffff;
      *(undefined4 *)(param_1 + 0x10) = 0;
      FUN_0043a230(param_1);
      return;
    }
    if ((DAT_006b28b0 & 0x2000) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0xa0) = DAT_006b28cc + 40000;
      FUN_0041da30();
      DAT_006b28b0 = DAT_006b28b0 & 0xffffdfff;
      return;
    }
    if ((DAT_006b28b0 & 1) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x8c) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xfffffffe;
      return;
    }
    if ((DAT_006b28b0 & 0x10) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x90) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xffffffef;
      return;
    }
    if ((DAT_006b28b0 & 0x20) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x98) = DAT_006b28cc + 10;
      DAT_006b28b0 = DAT_006b28b0 & 0xffffffdf;
      return;
    }
    if ((DAT_006b28b0 & 0x100) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x94) = DAT_006b28cc + 0x32;
      DAT_006b28b0 = DAT_006b28b0 & 0xfffffeff;
      return;
    }
    if ((DAT_006b28b0 & 0x200) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x9c) = DAT_006b28cc + 0x32;
      DAT_006b28b0 = DAT_006b28b0 & 0xfffffdff;
      return;
    }
    if ((DAT_006b28b0 & 0x1000000) != 0) {
      *(int *)(*(int *)(param_1 + 0x168) + 0x13c) = DAT_006b28cc + 5;
      DAT_006b28b0 = DAT_006b28b0 & 0xfeffffff;
      return;
    }
    if ((DAT_006b28b0 & 0x2000000) != 0) {
      uVar1 = DAT_006b28b8 + 0x4a;
      if (uVar1 < 0x4c) {
        uVar1 = 0x4c;
      }
      else if (0x4e < uVar1) {
        uVar1 = 0x4e;
        DAT_006b28b8 = 0;
      }
      *(int *)(*(int *)(param_1 + 0x168) + uVar1 * 4) = DAT_006b28cc + 5;
      DAT_006b28b0 = DAT_006b28b0 & 0xfdffffff;
      return;
    }
    if ((DAT_006b28b0 & 0x80000000) != 0) {
      uVar1 = FUN_0041cd80(0xc);
      if (uVar1 < 6) {
        *(int *)(*(int *)(param_1 + 0x168) + 0x140 + uVar1 * 4) = DAT_006b28cc + 40000;
      }
      DAT_006b28bc = 1;
      DAT_006b28b0 = DAT_006b28b0 & 0x7fffffff;
      return;
    }
  }
  return;
}

