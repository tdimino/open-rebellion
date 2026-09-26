
void __thiscall FUN_005f5cf0(void *this,void *param_1)

{
  short sVar1;
  void *pvVar2;
  void *this_00;
  int *piVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006557bb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4dd0(param_1,&param_1);
  sVar1 = (short)param_1;
  while (sVar1 != 0) {
    param_1 = (void *)((int)param_1 + 0xffff);
    this_00 = (void *)FUN_00618b70(0x20);
    local_4 = 0;
    if (this_00 == (void *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = FUN_005f5f10(this_00,0);
    }
    local_4 = 0xffffffff;
    if (piVar3 != (int *)0x0) {
      (**(code **)(*piVar3 + 4))(pvVar2);
      FUN_005f5440(this,piVar3);
    }
    sVar1 = (short)param_1;
  }
  ExceptionList = local_c;
  return;
}

