
int __thiscall FUN_005445d0(void *this,void *param_1)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  int iVar6;
  int iVar7;
  void *local_74;
  uint local_70;
  uint local_6c;
  uint local_68;
  uint local_64;
  uint local_60;
  int local_5c;
  int local_58;
  int local_54;
  int local_50;
  int local_4c;
  int local_48;
  int local_44;
  int local_40;
  int local_3c;
  undefined1 local_38 [4];
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647460;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_60);
  bVar2 = false;
  local_4 = 0;
  uVar3 = FUN_005434b0(&local_5c,&local_4c,&local_60);
  FUN_004ece30(&local_64);
  local_4._0_1_ = 1;
  uVar4 = FUN_00543550(&local_58,&local_48,&local_64);
  if ((uVar4 == 0) || (uVar3 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_004ece30(&local_68);
  local_4._0_1_ = 2;
  uVar3 = FUN_005435f0(&local_54,&local_44,&local_68);
  if ((uVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_004ece30(&local_6c);
  local_4._0_1_ = 3;
  uVar3 = FUN_00543690(&local_50,&local_40,&local_6c);
  if ((uVar3 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_004ece30(&local_70);
  local_4._0_1_ = 4;
  uVar3 = FUN_00543760(&local_74,&local_3c,&local_70);
  if ((uVar3 == 0) || (!bVar1)) {
    iVar7 = 0;
  }
  else {
    iVar7 = 1;
  }
  if ((((((((local_5c != 0) && (local_4c != 0)) &&
          (puVar5 = FUN_004ece40(&local_60), puVar5 != (uint *)0x0)) &&
         ((local_58 != 0 && (local_48 != 0)))) &&
        (puVar5 = FUN_004ece40(&local_64), puVar5 != (uint *)0x0)) &&
       (((local_54 != 0 && (local_44 != 0)) &&
        ((puVar5 = FUN_004ece40(&local_68), puVar5 != (uint *)0x0 &&
         (((local_50 != 0 && (local_40 != 0)) &&
          (puVar5 = FUN_004ece40(&local_6c), puVar5 != (uint *)0x0)))))))) &&
      ((local_74 != (void *)0x0 && (local_3c != 0)))) &&
     (puVar5 = FUN_004ece40(&local_70), puVar5 != (uint *)0x0)) {
    if ((((*(byte *)(local_5c + 0xac) & 1) != 0) && ((*(byte *)(local_58 + 0xac) & 1) != 0)) &&
       (((*(byte *)(local_54 + 0xac) & 1) != 0 &&
        (((*(byte *)(local_50 + 0xac) & 1) != 0 || ((*(byte *)(local_50 + 0x50) & 8) != 0)))))) {
      bVar2 = true;
    }
    uVar3 = 0;
    iVar6 = thunk_FUN_00506ed0();
    if ((iVar6 == 0) || (iVar7 == 0)) {
      iVar7 = 0;
    }
    else {
      iVar7 = 1;
    }
    if (iVar6 != 0) {
      FUN_004f2640(local_2c,iVar6,1,2);
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_00513120((int)local_2c);
      for (; (local_10 != 0 && (uVar3 == 0)); uVar3 = uVar3 & 1) {
        iVar6 = FUN_0052bed0((int)local_2c);
        uVar3 = *(uint *)(iVar6 + 0xac);
        FUN_005130d0((int)local_2c);
      }
      local_4._1_3_ = (uint3)((uint)local_4 >> 8);
      local_4._0_1_ = 4;
      FUN_004f26c0(local_2c);
    }
    if (((uVar3 != 0) && (bVar2)) && (iVar6 = FUN_005439b0((int)local_74), iVar6 != 0)) {
      if ((*(byte *)((int)local_74 + 0x78) & 0x80) == 0) {
        iVar6 = FUN_00543d90(this,1,param_1);
        if ((iVar6 == 0) || (iVar7 == 0)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        iVar7 = FUN_00543d90(this,0,param_1);
        if ((iVar7 != 0) && (bVar2)) {
          iVar7 = 1;
          goto LAB_005449a2;
        }
      }
      else {
        puVar5 = (uint *)FUN_0042d170(local_74,local_38);
        local_34 = 0x73;
        local_30 = 0x74;
        local_4._0_1_ = 6;
        if ((*puVar5 >> 0x18 < 0x73) || (0x73 < *puVar5 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        local_4._0_1_ = 4;
        FUN_00619730();
        if ((bVar2) || ((*(byte *)((int)local_74 + 0x78) & 0x10) == 0)) goto LAB_005449a2;
        puVar5 = (uint *)FUN_0042d170(local_74,local_38);
        local_4._0_1_ = 7;
        iVar6 = FUN_004ece60(puVar5);
        local_4._0_1_ = 4;
        FUN_00619730();
        if (iVar6 == 0) goto LAB_005449a2;
        iVar6 = FUN_00534640(local_74,1,param_1);
        if ((iVar6 != 0) && (iVar7 != 0)) {
          iVar7 = 1;
          goto LAB_005449a2;
        }
      }
      iVar7 = 0;
    }
  }
LAB_005449a2:
  local_4._0_1_ = 3;
  FUN_00619730();
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return iVar7;
}

