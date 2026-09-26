
int __cdecl FUN_00619460(int *param_1,int param_2,DWORD param_3)

{
  uint uVar1;
  DWORD DVar2;
  DWORD *pDVar3;
  
  if (((param_1[3] & 0x83U) != 0) && (((param_3 == 0 || (param_3 == 1)) || (param_3 == 2)))) {
    param_1[3] = param_1[3] & 0xffffffef;
    if (param_3 == 1) {
      DVar2 = FUN_00619270(param_1);
      param_2 = param_2 + DVar2;
      param_3 = 0;
    }
    FUN_0061b320(param_1);
    uVar1 = param_1[3];
    if ((uVar1 & 0x80) == 0) {
      if ((((uVar1 & 1) != 0) && ((uVar1 & 8) != 0)) && ((uVar1 & 0x400) == 0)) {
        param_1[6] = 0x200;
      }
    }
    else {
      param_1[3] = uVar1 & 0xfffffffc;
    }
    DVar2 = FUN_0061bb80(param_1[4],param_2,param_3);
    return (DVar2 != 0xffffffff) - 1;
  }
  pDVar3 = FUN_0061c250();
  *pDVar3 = 0x16;
  return -1;
}

