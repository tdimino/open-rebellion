
int __cdecl
FUN_0053feb0(uint param_1,void *param_2,undefined4 *param_3,undefined4 *param_4,int param_5)

{
  uint uVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  
  if ((param_1 < 0x400) || (0x421 < param_1)) {
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
      FUN_004f26d0((void *)(param_1 + 0x24),param_3);
      FUN_004f26d0((void *)(uVar1 + 0x28),param_4);
      FUN_004fd3b0(uVar1);
    }
  }
  return iVar3;
}

