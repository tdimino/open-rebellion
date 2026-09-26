
undefined4 __thiscall FUN_00544da0(void *this,void *param_1)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  uint *puVar4;
  int iVar5;
  undefined4 uVar6;
  int iVar7;
  uint local_38;
  int local_34;
  uint local_30;
  uint local_2c;
  int local_28;
  int local_24;
  int local_20;
  int local_1c;
  int local_18;
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006474e8;
  local_c = ExceptionList;
  iVar7 = 0;
  iVar5 = 1;
  ExceptionList = &local_c;
  if ((*(uint *)((int)this + 0x58) & 0x2000) != 0) goto LAB_00544f82;
  ExceptionList = &local_c;
  FUN_004ece30(&local_38);
  local_4 = 0;
  FUN_004ece30(&local_2c);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_30);
  local_4 = CONCAT31(local_4._1_3_,2);
  uVar2 = FUN_005434b0(&local_34,&local_1c,&local_38);
  uVar3 = FUN_00543760(&local_28,&local_18,&local_2c);
  if ((uVar3 == 0) || (uVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  uVar2 = FUN_00543800(&local_24,local_14,&local_30);
  if ((uVar2 == 0) || (local_20 = 1, !bVar1)) {
    local_20 = 0;
  }
  if ((local_34 != 0) && (local_1c != 0)) {
    puVar4 = FUN_004ece40(&local_38);
    if ((puVar4 != (uint *)0x0) && ((local_28 != 0 && (local_18 != 0)))) {
      puVar4 = FUN_004ece40(&local_2c);
      if ((puVar4 != (uint *)0x0) && ((local_24 != 0 && (local_14[0] != 0)))) {
        puVar4 = FUN_004ece40(&local_30);
        if (puVar4 != (uint *)0x0) {
          iVar5 = FUN_005439e0(local_28);
          iVar7 = FUN_005439e0(local_24);
          if (((((*(byte *)(local_34 + 0xac) & 1) != 0) && ((*(uint *)(local_34 + 0x50) & 1) != 0))
              && ((*(byte *)(local_34 + 0x78) & 0xc0) == 0)) &&
             ((*(uint *)(local_34 + 0x50) & 0x1000) == 0)) {
            local_14[1] = 0x98000481;
            FUN_00619730();
            if (((local_38 != 0x98000481) && ((*(byte *)(local_34 + 0xb0) & 2) != 0)) &&
               ((iVar5 != 0 && (iVar7 != 0)))) {
              iVar7 = 1;
              goto LAB_00544f51;
            }
          }
          iVar7 = 0;
        }
      }
    }
  }
LAB_00544f51:
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar5 = local_20;
LAB_00544f82:
  iVar7 = FUN_00543ee0(this,iVar7,param_1);
  if ((iVar7 == 0) || (iVar5 == 0)) {
    uVar6 = 0;
  }
  else {
    uVar6 = 1;
  }
  ExceptionList = local_c;
  return uVar6;
}

