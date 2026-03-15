
undefined4 __fastcall FUN_005a21e0(int *param_1)

{
  bool bVar1;
  void *pvVar2;
  int iVar3;
  undefined3 extraout_var;
  void *this;
  undefined3 extraout_var_00;
  undefined4 uVar4;
  float in_stack_ffffffe4;
  float in_stack_ffffffe8;
  float in_stack_ffffffec;
  undefined4 in_stack_fffffff0;
  int iVar5;
  
  pvVar2 = (void *)FUN_005c6070(param_1 + 1);
  iVar5 = 0;
  FUN_005a2260(pvVar2,(undefined4 *)&stack0xffffffe4);
  iVar3 = FUN_005aaf90();
  bVar1 = FUN_005a61c0(iVar3);
  iVar3 = CONCAT31(extraout_var,bVar1);
  this = (void *)FUN_005aaf90();
  iVar3 = FUN_005aa970(this,iVar3,in_stack_ffffffe4,in_stack_ffffffe8,in_stack_ffffffec,
                       in_stack_fffffff0,iVar5);
  if (iVar3 == 0) {
    iVar5 = 1;
    FUN_005a2260(pvVar2,(undefined4 *)&stack0xffffffe4);
    iVar3 = FUN_005aaf90();
    bVar1 = FUN_005a61c0(iVar3);
    iVar3 = CONCAT31(extraout_var_00,bVar1);
    pvVar2 = (void *)FUN_005aaf90();
    iVar3 = FUN_005aa970(pvVar2,iVar3,in_stack_ffffffe4,in_stack_ffffffe8,in_stack_ffffffec,
                         in_stack_fffffff0,iVar5);
    if (iVar3 == 0) {
      return 0;
    }
  }
  uVar4 = (**(code **)(*param_1 + 0xc))();
  return uVar4;
}

