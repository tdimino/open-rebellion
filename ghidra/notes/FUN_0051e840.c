
uint __thiscall FUN_0051e840(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  int iVar3;
  uint uVar4;
  undefined3 extraout_var;
  int local_20 [3];
  int iStack_14;
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  uint uVar5;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006432f8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00568fe0(local_20);
  piVar1 = param_1;
  local_4 = 0;
  local_20[0] = (**(code **)(*param_1 + 0x10))();
  iVar3 = (**(code **)(*piVar1 + 0x10))();
  uVar5 = 0;
  if (iVar3 == 4) {
    local_20[0] = (**(code **)(*piVar1 + 0x10))();
    iStack_14 = piVar1[4];
    iStack_10 = piVar1[5];
    local_20[2] = piVar1[6];
    local_20[1] = 0;
    uVar4 = FUN_0051e910(this,local_20,&param_1);
    uVar5 = 0;
    if (uVar4 != 0) {
      piVar1[3] = (int)param_1;
      bVar2 = FUN_0051eaa0(this,piVar1);
      uVar5 = CONCAT31(extraout_var,bVar2);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar5;
}

