
int * __fastcall FUN_00477a40(void *param_1)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *pvVar4;
  uint *puVar5;
  undefined1 *this;
  int iVar6;
  int *piVar7;
  void *local_38;
  undefined4 local_34;
  int *local_30;
  int local_2c;
  undefined1 local_28 [4];
  undefined1 local_24 [4];
  undefined1 local_20 [4];
  undefined1 local_1c [4];
  undefined1 *local_18;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063312a;
  pvStack_c = ExceptionList;
  bVar1 = false;
  local_30 = (int *)0x0;
  local_2c = 0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  local_4 = 0;
  FUN_004ece30(&local_34);
  local_4._0_1_ = 1;
  pvVar2 = (void *)thunk_FUN_005f5060((int)param_1 + 0x2c);
  while ((local_38 = pvVar2, pvVar2 != (void *)0x0 && (local_2c == 0))) {
    puVar3 = FUN_00403040(pvVar2,local_28);
    iVar6 = *(int *)((int)param_1 + 0x58);
    local_18 = &stack0xffffffb0;
    local_4._0_1_ = 2;
    FUN_004f26d0(&stack0xffffffb0,puVar3);
    pvVar4 = FUN_00419e40((void *)(iVar6 + 0x78));
    local_4._0_1_ = 1;
    FUN_00619730();
    if ((pvVar4 != (void *)0x0) &&
       (((*(uint *)((int)pvVar4 + 0x38) & 0x3000) != 0 &&
        ((*(uint *)((int)pvVar4 + 0x38) & 0xf0000802) == 0)))) {
      FUN_004f26d0(&local_34,(undefined4 *)((int)pvVar4 + 0x28));
      pvVar4 = (void *)thunk_FUN_005f5060((int)pvVar4 + 0x48);
      pvVar2 = local_38;
      for (; pvVar4 != (void *)0x0; pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
        local_38 = pvVar2;
        puVar5 = FUN_00403040(pvVar4,local_24);
        local_4._0_1_ = 3;
        pvVar2 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x58) + 0x58),puVar5);
        local_4._0_1_ = 1;
        FUN_00619730();
        if (((pvVar2 != (void *)0x0) && ((*(uint *)((int)pvVar2 + 0x24) & 0x801000) != 0)) &&
           ((*(uint *)((int)pvVar2 + 0x24) & 0x800) == 0)) {
          local_2c = 1;
          puVar5 = FUN_00403040(pvVar4,local_20);
          local_4._0_1_ = 4;
          FUN_004790d0(param_1,puVar5);
          local_4._0_1_ = 1;
          FUN_00619730();
          this = (undefined1 *)FUN_00618b70(0x20);
          local_4._0_1_ = 5;
          local_18 = this;
          if (this == (undefined1 *)0x0) {
            pvVar2 = (void *)0x0;
          }
          else {
            puVar3 = FUN_00403040(pvVar4,local_1c);
            bVar1 = true;
            local_4 = CONCAT31(local_4._1_3_,6);
            pvVar2 = FUN_004f5b10(this,puVar3,0);
          }
          local_4 = 5;
          if (bVar1) {
            bVar1 = false;
            FUN_00619730();
          }
          local_4._0_1_ = 1;
          if (pvVar2 != (void *)0x0) {
            FUN_004f57b0(local_14,pvVar2);
          }
        }
        pvVar2 = local_38;
      }
    }
    pvVar2 = *(void **)((int)pvVar2 + 0x10);
  }
  iVar6 = thunk_FUN_005f5060((int)local_14);
  if ((iVar6 != 0) && (piVar7 = (int *)FUN_004f5cd0(0x270), local_30 = piVar7, piVar7 != (int *)0x0)
     ) {
    piVar7[8] = *(int *)((int)param_1 + 0x20);
    (**(code **)(*piVar7 + 0x24))();
    (**(code **)(*piVar7 + 0x2c))(&local_38);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return local_30;
}

