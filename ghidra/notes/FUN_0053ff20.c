
int __cdecl FUN_0053ff20(uint param_1,void *param_2,int param_3,int param_4,int param_5)

{
  uint uVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  
  if ((param_1 < 0x500) || (0x541 < param_1)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (iVar3 != 0) {
    bVar2 = FUN_005400f0(param_1,param_2,param_5,(int *)&param_1);
    uVar1 = param_1;
    iVar3 = CONCAT31(extraout_var,bVar2);
    if ((iVar3 != 0) && (param_1 != 0)) {
      FUN_005f3090((void *)(param_1 + 0x24),param_3);
      FUN_005f3090((void *)(uVar1 + 0x30),param_4);
      FUN_004fd3b0(uVar1);
    }
  }
  return iVar3;
}

