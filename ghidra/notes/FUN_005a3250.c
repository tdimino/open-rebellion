
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005a3250(int *param_1)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  undefined3 extraout_var;
  void *pvVar4;
  undefined3 extraout_var_00;
  undefined4 uVar5;
  float in_stack_ffffffc0;
  float in_stack_ffffffc4;
  float in_stack_ffffffc8;
  undefined4 in_stack_ffffffcc;
  int iVar6;
  float fStack_24;
  float fStack_20;
  float fStack_1c;
  float fStack_18;
  undefined4 uStack_14;
  float fStack_8;
  
  fStack_20 = 0.0;
  fStack_1c = 0.0;
  fStack_18 = 0.0;
  uStack_14 = 0xbf800000;
  fStack_24 = 0.0;
  for (piVar2 = (int *)(**(code **)(param_1[1] + 0x10))(); piVar2 != (int *)0x0;
      piVar2 = (int *)(**(code **)(*piVar2 + 8))()) {
    fStack_24 = fStack_24 - _DAT_0066bb50;
    iVar3 = (**(code **)(*piVar2 + 0x10))();
    fStack_8 = *(float *)(iVar3 + 0x38);
    uStack_14 = 0xbf800000;
    fStack_1c = *(float *)(iVar3 + 0x34) + fStack_1c;
    fStack_18 = fStack_8 + fStack_18;
    fStack_20 = *(float *)(iVar3 + 0x30) + fStack_20;
  }
  if (_DAT_0066bb48 < fStack_24) {
    fStack_24 = _DAT_0066bb54 / fStack_24;
    iVar6 = 0;
    uStack_14 = 0xbf800000;
    fStack_20 = fStack_24 * fStack_20;
    fStack_1c = fStack_24 * fStack_1c;
    fStack_18 = fStack_24 * fStack_18;
    FUN_005a33b0(&stack0xffffffc0,&fStack_20);
    iVar3 = FUN_005aaf90();
    bVar1 = FUN_005a61c0(iVar3);
    iVar3 = CONCAT31(extraout_var,bVar1);
    pvVar4 = (void *)FUN_005aaf90();
    iVar3 = FUN_005aa970(pvVar4,iVar3,in_stack_ffffffc0,in_stack_ffffffc4,in_stack_ffffffc8,
                         in_stack_ffffffcc,iVar6);
    if (iVar3 == 0) {
      iVar6 = 1;
      FUN_005a33b0(&stack0xffffffc0,&fStack_20);
      iVar3 = FUN_005aaf90();
      bVar1 = FUN_005a61c0(iVar3);
      iVar3 = CONCAT31(extraout_var_00,bVar1);
      pvVar4 = (void *)FUN_005aaf90();
      iVar3 = FUN_005aa970(pvVar4,iVar3,in_stack_ffffffc0,in_stack_ffffffc4,in_stack_ffffffc8,
                           in_stack_ffffffcc,iVar6);
      if (iVar3 == 0) {
        return 0;
      }
    }
    uVar5 = (**(code **)(*param_1 + 0xc))();
    return uVar5;
  }
  return 0;
}

