
int * __fastcall FUN_00475c70(void *param_1)

{
  void *this;
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  uint *puVar4;
  void *pvVar5;
  int iVar6;
  undefined4 *puVar7;
  int *local_2c;
  undefined4 local_28;
  undefined1 local_24 [4];
  undefined1 local_20 [4];
  undefined1 local_1c [4];
  void *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632d72;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_2c = (int *)0x0;
  bVar1 = false;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_28);
  local_4 = 0;
  FUN_004f4340(local_14);
  local_4._0_1_ = 1;
  pvVar3 = (void *)thunk_FUN_005f5060((int)param_1 + 0x24);
  while ((this = pvVar3, this != (void *)0x0 && (!bVar1))) {
    puVar4 = FUN_00403040(this,local_24);
    local_4._0_1_ = 2;
    pvVar5 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x4c) + 0x58),puVar4);
    local_4._0_1_ = 1;
    FUN_00619730();
    if ((pvVar5 != (void *)0x0) &&
       ((((*(uint *)((int)pvVar5 + 0x24) & 0x2800) == 0 &&
         (iVar6 = FUN_00484510((int)pvVar5), iVar6 == 0)) &&
        (bVar1 = true, *(int *)((int)param_1 + 0x34) < *(int *)((int)pvVar5 + 0x7c))))) {
      bVar1 = false;
    }
    if (bVar1) {
      pvVar3 = *(void **)((int)this + 0x10);
      FUN_004f26d0(&local_28,(undefined4 *)((int)pvVar5 + 0x2c));
      pvVar5 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 3;
      local_18 = pvVar5;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        puVar7 = FUN_00403040(this,local_20);
        bVar2 = true;
        local_4 = CONCAT31(local_4._1_3_,4);
        pvVar5 = FUN_004f5b10(pvVar5,puVar7,0);
      }
      local_4 = 3;
      if (bVar2) {
        bVar2 = false;
        FUN_00619730();
      }
      local_4._0_1_ = 1;
      if (pvVar5 != (void *)0x0) {
        FUN_004f57b0(local_14,pvVar5);
        puVar4 = FUN_00403040(this,local_1c);
        local_4._0_1_ = 5;
        FUN_00475f60(param_1,puVar4);
        local_4._0_1_ = 1;
        FUN_00619730();
      }
    }
    else {
      pvVar3 = *(void **)((int)this + 0x10);
    }
  }
  iVar6 = thunk_FUN_005f5060((int)local_14);
  if ((iVar6 != 0) && (local_2c = (int *)FUN_004f5cd0(0x270), local_2c != (int *)0x0)) {
    local_2c[8] = *(int *)((int)param_1 + 0x20);
    (**(code **)(*local_2c + 0x24))(local_14);
    (**(code **)(*local_2c + 0x2c))((int)param_1 + 0x30);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_2c;
}

