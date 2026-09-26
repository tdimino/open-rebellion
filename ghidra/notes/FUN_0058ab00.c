
bool __thiscall FUN_0058ab00(void *this,uint *param_1)

{
  uint *puVar1;
  uint *puVar2;
  int iVar3;
  bool bVar4;
  void *local_c;
  undefined1 *puStack_8;
  uint *local_4;
  
  puVar1 = param_1;
  local_4 = (uint *)0xffffffff;
  puStack_8 = &LAB_0064fd00;
  local_c = ExceptionList;
  bVar4 = true;
  ExceptionList = &local_c;
  FUN_004ece80(param_1);
  puVar2 = FUN_004ece40(puVar1);
  if (puVar2 == (uint *)0x0) {
    param_1 = (uint *)0x32000242;
    local_4 = puVar2;
    iVar3 = FUN_0058ac50((uint *)&param_1,(void *)((int)this + 0x50),puVar1);
    bVar4 = iVar3 != 0;
    local_4 = (uint *)0xffffffff;
    FUN_00619730();
  }
  puVar2 = FUN_004ece40(puVar1);
  if (puVar2 == (uint *)0x0) {
    param_1 = (uint *)0x35000281;
    local_4 = (uint *)0x1;
    iVar3 = FUN_0058ac50((uint *)&param_1,(void *)((int)this + 0x50),puVar1);
    if ((iVar3 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    local_4 = (uint *)0xffffffff;
    FUN_00619730();
  }
  puVar2 = FUN_004ece40(puVar1);
  if (puVar2 == (uint *)0x0) {
    iVar3 = FUN_0058aca0((int)this + 0x50,puVar1);
    if ((iVar3 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  puVar2 = FUN_004ece40(puVar1);
  if (puVar2 == (uint *)0x0) {
    iVar3 = FUN_0058aca0((int)this + 0x58,puVar1);
    if ((iVar3 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  ExceptionList = local_c;
  return bVar4;
}

