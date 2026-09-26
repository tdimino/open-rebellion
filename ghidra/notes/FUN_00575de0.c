
bool __thiscall FUN_00575de0(void *this,void *param_1,undefined4 param_2,int *param_3,void *param_4)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  uint *puVar4;
  int iVar5;
  int *this_00;
  bool bVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064ca10;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_3 = 1;
  param_3 = (int *)0x0;
  iVar2 = FUN_00586c80(this,(int *)&param_3);
  bVar6 = iVar2 != 0;
  this_00 = (int *)0x0;
  if (param_3 != (int *)0x0) {
    local_4 = 0;
    uVar3 = (**(code **)(*param_3 + 4))();
    if (((uVar3 < 0x33) || (0x33 < uVar3)) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar6) {
      this_00 = param_3;
    }
  }
  pvVar1 = param_4;
  if (this_00 != (int *)0x0) {
    iVar2 = FUN_00572690(this_00,1,param_4);
    if ((iVar2 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    puVar4 = FUN_004025b0(param_1,(uint *)&param_4);
    local_4 = 1;
    iVar2 = FUN_005349e0(this_00,puVar4,0,pvVar1);
    if ((iVar2 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    iVar2 = 2;
    if ((*(byte *)(this_00 + 0x2b) & 1) != 0) {
      iVar2 = 3;
      iVar5 = FUN_004eeb10(this_00,0,pvVar1);
      if ((iVar5 == 0) || (!bVar6)) {
        bVar6 = false;
      }
      else {
        bVar6 = true;
      }
      iVar5 = FUN_00572700(this_00,1,pvVar1);
      if ((iVar5 == 0) || (!bVar6)) {
        bVar6 = false;
      }
      else {
        bVar6 = true;
      }
    }
    iVar2 = FUN_00521880(this,iVar2,pvVar1);
    if ((iVar2 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
    iVar2 = FUN_00572690(this_00,0,pvVar1);
    if ((iVar2 == 0) || (!bVar6)) {
      bVar6 = false;
    }
    else {
      bVar6 = true;
    }
  }
  ExceptionList = pvStack_c;
  return bVar6;
}

