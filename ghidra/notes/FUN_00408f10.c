
void __fastcall FUN_00408f10(int param_1)

{
  bool bVar1;
  int *piVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  uint uVar5;
  char *pcVar6;
  uint *puVar7;
  undefined4 uVar8;
  undefined4 *this;
  int iVar9;
  int *piVar10;
  undefined1 uVar11;
  int local_a4;
  int *local_a0;
  undefined4 auStack_90 [3];
  int local_84 [6];
  void *pvStack_6c;
  undefined4 auStack_68 [3];
  undefined4 auStack_5c [6];
  int iStack_44;
  int iStack_3c;
  undefined4 local_34 [6];
  int local_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629b6e;
  pvStack_c = ExceptionList;
  if ((*(int *)(param_1 + 0x9c) == 4) || (local_a4 = 0, *(int *)(param_1 + 0x9c) == 5)) {
    local_a4 = 1;
  }
  local_a0 = local_84;
  piVar10 = (int *)(param_1 + 0xc4);
  iVar9 = 1;
  ExceptionList = &pvStack_c;
  do {
    piVar2 = (int *)FUN_00604500((void *)(param_1 + 0x6c),iVar9 + 0x78);
    if (piVar2 != (int *)0x0) {
      FUN_00401860(local_34,iVar9,DAT_006b1208);
      local_4 = 0;
      if (DAT_006b1208 == 0) {
        uVar5 = (uint)(local_1c == 1);
      }
      else {
        uVar5 = 0;
        if (local_1c == 2) {
          bVar1 = FUN_00411410(iVar9);
          uVar5 = CONCAT31(extraout_var,bVar1);
        }
      }
      local_4 = 0xffffffff;
      FUN_004118f0(local_34);
      if (uVar5 == 0) {
        (**(code **)(*piVar2 + 0x10))();
      }
      else {
        iVar3 = FUN_00401910(iVar9,local_a4);
        if (DAT_006b1208 == 0) {
          if (iVar3 == 0) {
            (**(code **)(*piVar2 + 0x10))();
          }
          else {
            (**(code **)(*piVar2 + 0xc))();
          }
        }
        else if ((local_a4 == 0) || (iVar3 == 0)) {
          (**(code **)(*piVar2 + 0x10))();
        }
        else {
          (**(code **)(*piVar2 + 0xc))();
        }
      }
    }
    if (*piVar10 != 0) {
      FUN_005f2f50(auStack_90);
      local_4 = 1;
      FUN_00401860(auStack_5c,iVar9,local_a4);
      local_4._0_1_ = 2;
      pvVar4 = FUN_00408d90(auStack_5c,auStack_68);
      local_4._0_1_ = 3;
      FUN_005f3090(auStack_90,(int)pvVar4);
      local_4._0_1_ = 2;
      FUN_005f2ff0(auStack_68);
      local_84[0] = 0x53;
      local_84[1] = 0x7d;
      local_84[2] = 0xa7;
      local_84[3] = 0xd1;
      local_84[4] = 0xfb;
      local_84[5] = 0x125;
      uVar5 = FUN_005f3070((int)auStack_90);
      if ((short)uVar5 == 0) {
        FUN_00604f90((void *)*piVar10,&DAT_006b120c);
      }
      else {
        pcVar6 = (char *)FUN_00583c40((int)auStack_90);
        FUN_00604f90((void *)*piVar10,pcVar6);
        if (iStack_44 == 2) {
          uVar5 = 0x2749;
        }
        else if (iStack_3c == 1) {
          uVar5 = 0x2748;
        }
        else {
          if (iStack_3c != 2) goto LAB_004091c4;
          uVar5 = 0x2747;
        }
        iVar3 = *local_a0;
        puVar7 = (uint *)FUN_005ff440(param_1);
        if (puVar7 != (uint *)0x0) {
          pvVar4 = (void *)FUN_00618b70(0x28);
          local_4._0_1_ = 4;
          pvStack_6c = pvVar4;
          if (pvVar4 == (void *)0x0) {
            this = (undefined4 *)0x0;
          }
          else {
            uVar11 = 2;
            uVar8 = FUN_006037f0(8);
            this = FUN_005fbd20(pvVar4,uVar8,uVar5,uVar11);
          }
          local_4._0_1_ = 2;
          if (this != (undefined4 *)0x0) {
            FUN_005fd0f0(this,puVar7,0x55,iVar3);
            FUN_005fbfa0(this);
            FUN_00618b60((undefined *)this);
          }
        }
      }
LAB_004091c4:
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_004118f0(auStack_5c);
      local_4 = 0xffffffff;
      FUN_005f2ff0(auStack_90);
    }
    piVar10 = piVar10 + 1;
    local_a0 = local_a0 + 1;
    bVar1 = 5 < iVar9;
    iVar9 = iVar9 + 1;
    if (bVar1) {
      ExceptionList = pvStack_c;
      return;
    }
  } while( true );
}

