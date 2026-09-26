
int * __thiscall FUN_004b1440(void *this,int param_1)

{
  bool bVar1;
  bool bVar2;
  void *this_00;
  uint *puVar3;
  void *pvVar4;
  int *piVar5;
  void *this_01;
  int *piVar6;
  int unaff_retaddr;
  undefined1 local_44 [4];
  undefined1 local_40 [4];
  uint local_3c;
  void *local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638f92;
  pvStack_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_34);
  piVar6 = (int *)0x0;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  bVar1 = false;
  if (param_1 != 0) {
    this_00 = (void *)thunk_FUN_005f5060(param_1 + 0x48);
    while ((this_00 != (void *)0x0 && (!bVar1))) {
      puVar3 = FUN_00403040(this_00,local_44);
      local_4._0_1_ = 1;
      pvVar4 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x58),puVar3);
      local_4._0_1_ = 0;
      FUN_00619730();
      bVar1 = false;
      if ((pvVar4 != (void *)0x0) && (0 < *(int *)((int)pvVar4 + 0x68))) {
        puVar3 = FUN_00403040(pvVar4,local_40);
        local_4._0_1_ = 2;
        piVar5 = FUN_004f2e20(*(int *)((int)this + 4),puVar3);
        local_4._0_1_ = 0;
        FUN_00619730();
        if (piVar5 != (int *)0x0) {
          FUN_00504c40(local_2c,piVar5,1);
          local_4._0_1_ = 3;
          FUN_00513120((int)local_2c);
          if ((local_10 != 0) && (piVar6 = (int *)FUN_004f5cd0(0x201), piVar6 != (int *)0x0)) {
            bVar1 = true;
            pvVar4 = (void *)FUN_00618b70(0x20);
            local_4._0_1_ = 4;
            local_38 = pvVar4;
            if (pvVar4 == (void *)0x0) {
              pvVar4 = (void *)0x0;
            }
            else {
              puVar3 = &local_3c;
              this_01 = (void *)FUN_0052bed0((int)local_2c);
              puVar3 = FUN_004025b0(this_01,puVar3);
              bVar2 = true;
              local_4 = CONCAT31(local_4._1_3_,5);
              pvVar4 = FUN_004f5b10(pvVar4,puVar3,0);
            }
            local_4 = 4;
            if (bVar2) {
              bVar2 = false;
              FUN_00619730();
            }
            local_4._0_1_ = 3;
            if (pvVar4 != (void *)0x0) {
              FUN_004f57b0(local_34,pvVar4);
              piVar6[8] = *(int *)((int)this + 4);
              (**(code **)(*piVar6 + 0x24))(local_34);
              (**(code **)(*piVar6 + 0x2c))(unaff_retaddr + 0x2c);
            }
          }
          local_4._0_1_ = 0;
          FUN_00504d40(local_2c);
        }
      }
      this_00 = *(void **)((int)this_00 + 0x10);
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_34);
  ExceptionList = pvStack_c;
  return piVar6;
}

