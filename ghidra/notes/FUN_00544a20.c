
int __thiscall FUN_00544a20(void *this,void *param_1)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  uint local_30;
  uint local_2c;
  uint local_28;
  int local_24;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647488;
  local_c = ExceptionList;
  iVar6 = 1;
  if ((*(uint *)((int)this + 0x58) & 0x1000) != 0) {
    ExceptionList = &local_c;
    FUN_004ece30(&local_2c);
    local_4 = 0;
    FUN_004ece30(&local_30);
    local_4._0_1_ = 1;
    FUN_004ece30(&local_28);
    local_4 = CONCAT31(local_4._1_3_,2);
    uVar2 = FUN_005434b0(&local_24,&local_20,&local_2c);
    uVar3 = FUN_00543760(&local_1c,&local_18,&local_30);
    if ((uVar3 == 0) || (uVar2 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar2 = FUN_00543800(&local_14,&local_10,&local_28);
    if ((uVar2 == 0) || (!bVar1)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    if ((local_24 != 0) && (local_20 != 0)) {
      puVar4 = FUN_004ece40(&local_2c);
      if ((puVar4 != (uint *)0x0) && ((local_1c != 0 && (local_18 != 0)))) {
        puVar4 = FUN_004ece40(&local_30);
        if ((puVar4 != (uint *)0x0) && ((local_14 != 0 && (local_10 != 0)))) {
          puVar4 = FUN_004ece40(&local_28);
          if ((puVar4 != (uint *)0x0) && (local_30 != local_2c)) {
            iVar5 = FUN_00543e00(this,1,param_1);
            if ((iVar5 == 0) || (iVar6 == 0)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            iVar6 = FUN_00543e00(this,0,param_1);
            if ((iVar6 == 0) || (!bVar1)) {
              iVar6 = 0;
            }
            else {
              iVar6 = 1;
            }
          }
        }
      }
    }
    local_4._0_1_ = 1;
    FUN_00619730();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar6;
}

