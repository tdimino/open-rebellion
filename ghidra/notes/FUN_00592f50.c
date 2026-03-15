
bool __thiscall FUN_00592f50(void *this,undefined4 param_1,int param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined4 uVar4;
  undefined3 extraout_var;
  void *pvVar5;
  uint *puVar6;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  void *pvVar7;
  bool bVar8;
  undefined1 *puVar9;
  void **ppvVar10;
  int *piStack_84;
  void *local_80;
  uint uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined1 auStack_70 [4];
  undefined4 auStack_6c [10];
  int iStack_44;
  undefined1 auStack_40 [4];
  undefined4 auStack_3c [10];
  int iStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00650930;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_80 = this;
  iVar3 = FUN_00524b70(this,param_1,param_2,param_3);
  bVar8 = iVar3 != 0;
  if (param_2 == 10) {
                    /* WARNING: Load size is inaccurate */
    iVar3 = (**(code **)(*this + 0x278))(param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    FUN_004f4340(&uStack_78);
    puStack_8 = (undefined1 *)0x0;
    FUN_00526090(auStack_70,this);
    puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
    FUN_00525930((int)auStack_70);
    while (iStack_44 != 0) {
      ppvVar10 = &local_80;
      bVar1 = false;
      local_80 = (void *)0x0;
      uVar4 = FUN_00525f20((int)auStack_70);
      bVar2 = FUN_00593320(this,uVar4,ppvVar10);
      if ((CONCAT31(extraout_var,bVar2) != 0) && (bVar8)) {
        bVar1 = true;
      }
      puVar6 = &uStack_7c;
      pvVar5 = (void *)FUN_00525f20((int)auStack_70);
      puVar6 = FUN_004025b0(pvVar5,puVar6);
      puStack_8._0_1_ = 2;
      bVar8 = FUN_004f44b0(&uStack_78,puVar6,local_80);
      if ((CONCAT31(extraout_var_00,bVar8) == 0) || (!bVar1)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
      FUN_00619730();
      FUN_005258f0((int)auStack_70);
      param_3 = (void *)0xa;
    }
    while (pvVar5 = (void *)thunk_FUN_005f5060((int)&uStack_78), pvVar7 = pvVar5,
          pvVar5 != (void *)0x0) {
      do {
        if (*(int *)((int)pvVar5 + 0x1c) < *(int *)((int)pvVar7 + 0x1c)) {
          pvVar7 = pvVar5;
        }
        pvVar5 = *(void **)((int)pvVar5 + 0x10);
      } while (pvVar5 != (void *)0x0);
      puVar6 = (uint *)FUN_00403040(pvVar7,&uStack_7c);
      puStack_8._0_1_ = 3;
      pvVar5 = (void *)FUN_00505190(puVar6);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
      FUN_00619730();
      if ((pvVar5 == (void *)0x0) || (!bVar8)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
      if (pvVar5 != (void *)0x0) {
        iVar3 = (**(code **)(*piStack_84 + 0x27c))
                          (pvVar5,*(undefined4 *)((int)pvVar7 + 0x1c),&stack0xffffff78,10);
        if ((iVar3 == 0) || (!bVar8)) {
          bVar8 = false;
        }
        else {
          bVar8 = true;
        }
      }
      puVar6 = FUN_004025b0(pvVar5,(uint *)&local_80);
      puStack_8._0_1_ = 4;
      FUN_004f58e0(&uStack_78,puVar6);
      puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
      FUN_00619730();
      param_3 = (void *)0xa;
      this = piStack_84;
    }
    FUN_00525e70(auStack_40,this);
    puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,5);
    FUN_00525930((int)auStack_40);
    while (bVar1 = false, iStack_14 != 0) {
      ppvVar10 = &local_80;
      local_80 = (void *)0x0;
      uVar4 = FUN_00525f20((int)auStack_40);
      bVar2 = FUN_00593320(this,uVar4,ppvVar10);
      if ((CONCAT31(extraout_var_01,bVar2) != 0) && (bVar8)) {
        bVar1 = true;
      }
                    /* WARNING: Load size is inaccurate */
      iVar3 = *this;
      puVar9 = &stack0xffffff78;
      pvVar5 = local_80;
      pvVar7 = param_3;
      uVar4 = FUN_00525f20((int)auStack_40);
      iVar3 = (**(code **)(iVar3 + 0x27c))(uVar4,pvVar5,puVar9,pvVar7);
      if ((iVar3 == 0) || (!bVar1)) {
        bVar8 = false;
      }
      else {
        bVar8 = true;
      }
      FUN_005258f0((int)auStack_40);
    }
                    /* WARNING: Load size is inaccurate */
    iVar3 = (**(code **)(*this + 0x280))(param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
                    /* WARNING: Load size is inaccurate */
    iVar3 = (**(code **)(*this + 0x284))(param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
    iStack_4._0_1_ = 1;
    FUN_00525f10(auStack_3c);
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_00526130(auStack_6c);
    iStack_4 = 0xffffffff;
    FUN_004f4380(&uStack_74);
  }
  if (param_2 == 0xb) {
                    /* WARNING: Load size is inaccurate */
    iVar3 = (**(code **)(*this + 0x284))(param_3);
    if ((iVar3 == 0) || (!bVar8)) {
      bVar8 = false;
    }
    else {
      bVar8 = true;
    }
  }
  ExceptionList = pvStack_c;
  return bVar8;
}

