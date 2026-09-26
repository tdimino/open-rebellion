
undefined4 __thiscall FUN_00536490(void *this,uint param_1,void *param_2)

{
  uint uVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  undefined4 uVar5;
  void *pvVar6;
  bool bVar7;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  uVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645b50;
  local_c = ExceptionList;
  if (param_1 == 0) {
    ExceptionList = &local_c;
    piVar2 = (int *)FUN_004ece30(&param_1);
    pvVar6 = param_2;
    local_4 = uVar1;
    iVar3 = FUN_00534390(this,piVar2,param_2);
    bVar7 = iVar3 != 0;
  }
  else {
    ExceptionList = &local_c;
    FUN_004ece30(&param_1);
    local_4 = 1;
    iVar3 = FUN_004f6b70(this,&param_1);
    pvVar6 = param_2;
    iVar4 = FUN_00534390(this,(int *)&param_1,param_2);
    if ((iVar4 == 0) || (iVar3 == 0)) {
      bVar7 = false;
    }
    else {
      bVar7 = true;
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar3 = FUN_00534870(this,0,pvVar6);
  if ((iVar3 == 0) || (!bVar7)) {
    bVar7 = false;
  }
  else {
    bVar7 = true;
  }
  iVar3 = FUN_005348e0(this,0,pvVar6);
  if ((iVar3 == 0) || (!bVar7)) {
    bVar7 = false;
  }
  else {
    bVar7 = true;
  }
  iVar3 = FUN_0053f9c0(0x312,this,uVar1,pvVar6);
  if ((iVar3 == 0) || (!bVar7)) {
    uVar5 = 0;
  }
  else {
    uVar5 = 1;
  }
  ExceptionList = local_c;
  return uVar5;
}

