
void FUN_00525b00(void *param_1,undefined4 *param_2)

{
  undefined4 *this;
  int iVar1;
  void *pvVar2;
  undefined4 *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  this = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643e20;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004f26d0(&param_2,param_2);
  local_4 = 0;
  FUN_004ece80(this);
  iVar1 = FUN_004ece60((uint *)&param_2);
  if (iVar1 == 0) {
    pvVar2 = (void *)thunk_FUN_005f5060((int)param_1);
  }
  else {
    pvVar2 = FUN_004f5940(param_1,(uint *)&param_2);
    if (pvVar2 == (void *)0x0) goto LAB_00525b8b;
    pvVar2 = *(void **)((int)pvVar2 + 0x10);
  }
  if (pvVar2 != (void *)0x0) {
    puVar3 = FUN_00403040(pvVar2,&param_1);
    local_4._0_1_ = 1;
    FUN_004f26d0(this,puVar3);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
LAB_00525b8b:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

