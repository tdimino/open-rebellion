
int * __thiscall FUN_005cdf50(void *this,int param_1,float param_2)

{
  float fVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  void *pvVar5;
  undefined4 uVar6;
  undefined **local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined2 local_74;
  int local_70;
  float local_6c [2];
  float local_64;
  float local_5c [2];
  float local_54;
  float local_4c [4];
  undefined1 local_3c [4];
  float local_38;
  float local_2c [4];
  undefined1 local_1c [4];
  float local_18;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654415;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = (int *)FUN_00618b70(0x10);
  local_4 = 0;
  if (piVar2 == (int *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    FUN_005f5830(piVar2);
    *piVar2 = (int)&PTR_FUN_0066c668;
  }
  local_84 = 0;
  local_80 = 0;
  local_7c = 0;
  local_78 = 0;
  local_74 = 0;
  local_88 = &PTR_FUN_0066d04c;
  local_70 = param_1;
  local_4 = 1;
  for (iVar3 = FUN_005ce950(this,param_1); iVar3 != 0; iVar3 = FUN_005ee480(iVar3)) {
    FUN_005a33b0(local_2c,(undefined4 *)(local_70 + 0x30));
    FUN_005a33b0(local_4c,(undefined4 *)(*(int *)(iVar3 + 0x18) + 0x30));
    if (param_2 < local_2c[0] - local_4c[0]) break;
    uVar6 = *(undefined4 *)(iVar3 + 0x18);
    FUN_005aaf90();
    iVar4 = FUN_005ad870(uVar6);
    if (((iVar4 != 0) && (iVar4 = *(int *)(iVar3 + 0x18), *(int *)(iVar4 + 0x170) != 6)) &&
       (*(int *)(iVar4 + 0x168) != *(int *)(param_1 + 0x168))) {
      FUN_005a33b0(local_3c,(undefined4 *)(iVar4 + 0x30));
      fVar1 = local_38;
      FUN_005a33b0(local_1c,(undefined4 *)(local_70 + 0x30));
      if (ABS(fVar1 - local_18) <= param_2) {
        FUN_005a33b0(local_6c,(undefined4 *)(*(int *)(iVar3 + 0x18) + 0x30));
        fVar1 = local_64;
        FUN_005a33b0(local_5c,(undefined4 *)(local_70 + 0x30));
        if (ABS(fVar1 - local_54) <= param_2) {
          pvVar5 = (void *)FUN_00618b70(0x14);
          local_4._0_1_ = 2;
          if (pvVar5 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            pvVar5 = FUN_005d0c50(pvVar5,*(int *)(iVar3 + 0x18));
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          (**(code **)(*piVar2 + 8))(pvVar5);
        }
      }
    }
  }
  for (iVar3 = FUN_005ce9c0(this,param_1); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
    FUN_005a33b0(local_5c,(undefined4 *)(*(int *)(iVar3 + 0x18) + 0x30));
    FUN_005a33b0(local_6c,(undefined4 *)(local_70 + 0x30));
    if (param_2 < local_5c[0] - local_6c[0]) break;
    uVar6 = *(undefined4 *)(iVar3 + 0x18);
    FUN_005aaf90();
    iVar4 = FUN_005ad870(uVar6);
    if (((iVar4 != 0) && (iVar4 = *(int *)(iVar3 + 0x18), *(int *)(iVar4 + 0x170) != 6)) &&
       ((*(int *)(iVar4 + 0x168) != *(int *)(param_1 + 0x168) &&
        ((ABS(*(float *)(iVar4 + 0x34) - *(float *)(local_70 + 0x34)) <= param_2 &&
         (ABS(*(float *)(iVar4 + 0x38) - *(float *)(local_70 + 0x38)) <= param_2)))))) {
      pvVar5 = (void *)FUN_00618b70(0x14);
      local_4._0_1_ = 3;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        pvVar5 = FUN_005d0c50(pvVar5,*(int *)(iVar3 + 0x18));
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      (**(code **)(*piVar2 + 8))(pvVar5);
    }
  }
  local_4 = 0xffffffff;
  FUN_005ee360(&local_88);
  ExceptionList = local_c;
  return piVar2;
}

