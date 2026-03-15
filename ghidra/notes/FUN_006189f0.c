
int __cdecl FUN_006189f0(int param_1,int param_2,int param_3,uint *param_4,uint *param_5)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  
  iVar2 = *(int *)(param_1 + 0x10);
  uVar3 = *(uint *)(param_1 + 0xc);
  uVar4 = uVar3;
  uVar5 = uVar3;
  if (-1 < param_2) {
    do {
      if (uVar4 == 0xffffffff) {
        FUN_0061d540();
      }
      uVar4 = uVar4 - 1;
      iVar1 = iVar2 + uVar4 * 0x14;
      if (((*(int *)(iVar1 + 4) < param_3) && (param_3 <= *(int *)(iVar1 + 8))) ||
         (uVar4 == 0xffffffff)) {
        param_2 = param_2 + -1;
        uVar3 = uVar5;
        uVar5 = uVar4;
      }
    } while (-1 < param_2);
  }
  uVar4 = uVar4 + 1;
  *param_4 = uVar4;
  *param_5 = uVar3;
  if ((*(uint *)(param_1 + 0xc) < uVar3) || (uVar3 < uVar4)) {
    FUN_0061d540();
  }
  return iVar2 + uVar4 * 0x14;
}

