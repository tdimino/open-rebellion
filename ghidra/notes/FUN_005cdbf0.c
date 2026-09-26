
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

int * __thiscall FUN_005cdbf0(void *this,int *param_1,float param_2)

{
  float fVar1;
  int iVar2;
  float fVar3;
  float fVar4;
  int iVar5;
  void *pvVar6;
  float10 fVar7;
  float fStack_a4;
  int *local_98;
  undefined **local_88;
  undefined4 local_84;
  undefined4 local_80;
  undefined4 local_7c;
  undefined4 local_78;
  undefined2 local_74;
  int *local_70;
  float afStack_6c [2];
  float fStack_64;
  float afStack_5c [2];
  float fStack_54;
  undefined1 auStack_4c [4];
  float fStack_48;
  float afStack_3c [4];
  float afStack_2c [4];
  undefined1 auStack_1c [4];
  float fStack_18;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006543d5;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_98 = (int *)FUN_00618b70(0x10);
  local_4 = 0;
  if (local_98 == (int *)0x0) {
    local_98 = (int *)0x0;
  }
  else {
    FUN_005f5830(local_98);
    *local_98 = (int)&PTR_FUN_0066c668;
  }
  local_88 = &PTR_FUN_0066d04c;
  local_84 = 0;
  local_80 = 0;
  local_7c = 0;
  local_78 = 0;
  local_74 = 0;
  local_70 = param_1;
  local_4 = 1;
  iVar5 = FUN_005ce950(this,param_1);
  fVar7 = (float10)(**(code **)(*param_1 + 0x8c))();
  fVar1 = (float)fVar7;
  fVar3 = param_2 * _DAT_0066d03c + (float)param_1[2];
  if (iVar5 != 0) {
    fVar7 = (float10)(**(code **)(**(int **)(iVar5 + 0x18) + 0x8c))();
    fVar7 = fVar7 + (float10)fVar1;
    while (iVar5 != 0) {
      fStack_a4 = (float)fVar7;
      FUN_005a33b0(afStack_3c,local_70 + 0xc);
      FUN_005a33b0(afStack_2c,(undefined4 *)(*(int *)(iVar5 + 0x18) + 0x30));
      if (fStack_a4 < afStack_3c[0] - afStack_2c[0]) break;
      FUN_005a33b0(auStack_4c,(undefined4 *)(*(int *)(iVar5 + 0x18) + 0x30));
      fVar4 = fStack_48;
      FUN_005a33b0(auStack_1c,local_70 + 0xc);
      if (ABS(fVar4 - fStack_18) <= *(float *)(*(int *)(iVar5 + 0x18) + 8) + fVar3) {
        FUN_005a33b0(afStack_6c,(undefined4 *)(*(int *)(iVar5 + 0x18) + 0x30));
        fVar4 = fStack_64;
        FUN_005a33b0(afStack_5c,local_70 + 0xc);
        if (ABS(fVar4 - fStack_54) <= fStack_a4) {
          pvVar6 = (void *)FUN_00618b70(0x14);
          local_4._0_1_ = 2;
          if (pvVar6 == (void *)0x0) {
            pvVar6 = (void *)0x0;
          }
          else {
            pvVar6 = FUN_005d0c50(pvVar6,*(int *)(iVar5 + 0x18));
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          (**(code **)(*local_98 + 8))(pvVar6);
        }
      }
      iVar5 = FUN_005ee480(iVar5);
      if (iVar5 == 0) break;
      fVar7 = (float10)(**(code **)(**(int **)(iVar5 + 0x18) + 0x8c))();
      fVar7 = fVar7 + (float10)fVar1;
    }
  }
  iVar5 = FUN_005ce9c0(this,param_1);
  if (iVar5 != 0) {
    fVar7 = (float10)(**(code **)(**(int **)(iVar5 + 0x18) + 0x8c))();
    fVar7 = fVar7 + (float10)fVar1;
    do {
      fStack_a4 = (float)fVar7;
      FUN_005a33b0(afStack_5c,(undefined4 *)(*(int *)(iVar5 + 0x18) + 0x30));
      FUN_005a33b0(afStack_6c,local_70 + 0xc);
      if (fStack_a4 < afStack_5c[0] - afStack_6c[0]) break;
      iVar2 = *(int *)(iVar5 + 0x18);
      if ((ABS(*(float *)(iVar2 + 0x34) - (float)local_70[0xd]) <= *(float *)(iVar2 + 8) + fVar3) &&
         (ABS(*(float *)(iVar2 + 0x38) - (float)local_70[0xe]) <= fStack_a4)) {
        pvVar6 = (void *)FUN_00618b70(0x14);
        local_4._0_1_ = 3;
        if (pvVar6 == (void *)0x0) {
          pvVar6 = (void *)0x0;
        }
        else {
          pvVar6 = FUN_005d0c50(pvVar6,*(int *)(iVar5 + 0x18));
        }
        local_4 = CONCAT31(local_4._1_3_,1);
        (**(code **)(*local_98 + 8))(pvVar6);
      }
      iVar5 = *(int *)(iVar5 + 0x10);
      if (iVar5 == 0) break;
      fVar7 = (float10)(**(code **)(**(int **)(iVar5 + 0x18) + 0x8c))();
      fVar7 = fVar7 + (float10)fVar1;
    } while (iVar5 != 0);
  }
  local_4 = 0xffffffff;
  FUN_005ee360(&local_88);
  ExceptionList = pvStack_c;
  return local_98;
}

