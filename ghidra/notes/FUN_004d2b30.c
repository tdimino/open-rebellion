
int * __thiscall FUN_004d2b30(void *this,void *param_1)

{
  ushort uVar1;
  bool bVar2;
  uint *puVar3;
  int *piVar4;
  void *pvVar5;
  undefined4 *puVar6;
  short local_3c [2];
  uint local_38;
  uint local_34;
  void *local_30;
  undefined4 local_2c [2];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063c1ca;
  pvStack_c = ExceptionList;
  local_38 = 0;
  ExceptionList = &pvStack_c;
  FUN_005f4950(&local_34,0);
  local_4 = 0;
  FUN_005f2f50(local_24);
  local_4._0_1_ = 1;
  FUN_004f4340(local_2c);
  bVar2 = false;
  local_4._0_1_ = 2;
  local_34 = *(uint *)((int)param_1 + 0x24);
  if ((local_34 & 0x10) != 0) {
    uVar1 = *(ushort *)((int)this + 0x44);
    if (*(int *)((int)this + 0x30) == 1) {
      if (0x13 < uVar1) goto LAB_004d2d89;
      local_3c[0] = uVar1 + 0x5200;
    }
    else {
      if (0x26 < uVar1) goto LAB_004d2d89;
      local_3c[0] = uVar1 + 0x5100;
    }
    goto LAB_004d2d77;
  }
  if ((local_34 & 0x20) == 0) {
    if ((local_34 & 0x40) == 0) goto LAB_004d2d89;
    uVar1 = *(ushort *)((int)this + 0x48);
    if (*(int *)((int)this + 0x30) == 1) {
      if (uVar1 < 0x10) {
        local_3c[0] = uVar1 + 0x52c0;
LAB_004d2d1a:
        local_3c[1] = 2;
        FUN_005f3010(local_24,local_3c);
        *(short *)((int)this + 0x48) = *(short *)((int)this + 0x48) + 1;
        goto LAB_004d2d84;
      }
      if (*(ushort *)((int)this + 0x46) < 0x23) {
        local_3c[0] = *(ushort *)((int)this + 0x46) + 0x5260;
        goto LAB_004d2d49;
      }
      if (0x13 < *(ushort *)((int)this + 0x44)) goto LAB_004d2d89;
      local_3c[0] = *(ushort *)((int)this + 0x44) + 0x5200;
    }
    else {
      if (uVar1 < 0x20) {
        local_3c[0] = uVar1 + 0x51c0;
        goto LAB_004d2d1a;
      }
      if (*(ushort *)((int)this + 0x46) < 0x22) {
        local_3c[0] = *(ushort *)((int)this + 0x46) + 0x5160;
        goto LAB_004d2d49;
      }
      if (0x26 < *(ushort *)((int)this + 0x44)) goto LAB_004d2d89;
      local_3c[0] = *(ushort *)((int)this + 0x44) + 0x5100;
    }
LAB_004d2d77:
    local_3c[1] = 2;
    FUN_005f3010(local_24,local_3c);
    *(short *)((int)this + 0x44) = *(short *)((int)this + 0x44) + 1;
  }
  else {
    uVar1 = *(ushort *)((int)this + 0x46);
    if (*(int *)((int)this + 0x30) != 1) {
      if (uVar1 < 0x22) {
        local_3c[0] = uVar1 + 0x5160;
        goto LAB_004d2d49;
      }
      if (0x26 < *(ushort *)((int)this + 0x44)) goto LAB_004d2d89;
      local_3c[0] = *(ushort *)((int)this + 0x44) + 0x5100;
      goto LAB_004d2d77;
    }
    if (0x22 < uVar1) {
      if (0x13 < *(ushort *)((int)this + 0x44)) goto LAB_004d2d89;
      local_3c[0] = *(ushort *)((int)this + 0x44) + 0x5200;
      goto LAB_004d2d77;
    }
    local_3c[0] = uVar1 + 0x5260;
LAB_004d2d49:
    local_3c[1] = 2;
    FUN_005f3010(local_24,local_3c);
    *(short *)((int)this + 0x46) = *(short *)((int)this + 0x46) + 1;
  }
LAB_004d2d84:
  bVar2 = true;
LAB_004d2d89:
  if (!bVar2) {
    puVar3 = (uint *)FUN_00403040(param_1,local_3c);
    local_4._0_1_ = 3;
    piVar4 = FUN_004f2e20(*(int *)((int)this + 0x30),puVar3);
    local_4._0_1_ = 2;
    FUN_00619730();
    if (piVar4 != (int *)0x0) {
      pvVar5 = FUN_004f6270(piVar4,local_18);
      local_4._0_1_ = 4;
      FUN_005f3090(local_24,(int)pvVar5);
      local_4._0_1_ = 2;
      FUN_005f2ff0(local_18);
    }
  }
  piVar4 = (int *)FUN_004f5cd0(0x203);
  if (piVar4 != (int *)0x0) {
    pvVar5 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 5;
    local_30 = pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      puVar6 = (undefined4 *)FUN_00403040(param_1,local_3c);
      local_38 = 1;
      local_4 = CONCAT31(local_4._1_3_,6);
      pvVar5 = FUN_004f5b10(pvVar5,puVar6,0);
    }
    local_4 = 5;
    if ((local_38 & 1) != 0) {
      FUN_00619730();
    }
    local_4._0_1_ = 2;
    if (pvVar5 != (void *)0x0) {
      FUN_004f57b0(local_2c,pvVar5);
      piVar4[8] = *(int *)((int)this + 0x30);
      (**(code **)(*piVar4 + 0x24))(local_2c);
      FUN_005f3090(piVar4 + 0x11,(int)local_24);
    }
  }
  local_4._0_1_ = 1;
  FUN_004f4380(local_2c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_24);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return piVar4;
}

