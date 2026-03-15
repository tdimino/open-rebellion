
int * __thiscall FUN_004b1330(void *this,void *param_1,void *param_2)

{
  void *this_00;
  void *this_01;
  undefined4 *puVar1;
  void *pvVar2;
  int *piVar3;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638f3a;
  pvStack_c = ExceptionList;
  piVar3 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  this_00 = param_1;
  local_4 = 0;
  if (param_1 != (void *)0x0) {
    piVar3 = (int *)FUN_004f5cd0((int)param_2);
    if (piVar3 != (int *)0x0) {
      this_01 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 1;
      param_2 = this_01;
      if (this_01 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        puVar1 = (undefined4 *)FUN_00403040(this_00,&param_1);
        local_4 = CONCAT31(local_4._1_3_,2);
        pvVar2 = FUN_004f5b10(this_01,puVar1,0);
      }
      local_4 = 1;
      if (this_01 != (void *)0x0) {
        FUN_00619730();
      }
      local_4 = local_4 & 0xffffff00;
      if (pvVar2 != (void *)0x0) {
        FUN_004f57b0(local_14,pvVar2);
        piVar3[8] = *(int *)((int)this + 4);
        (**(code **)(*piVar3 + 0x24))(local_14);
        (**(code **)(*piVar3 + 0x2c))((int)this_00 + 0x2c);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return piVar3;
}

