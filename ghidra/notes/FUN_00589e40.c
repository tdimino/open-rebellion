
bool __thiscall FUN_00589e40(void *this,uint param_1)

{
  uint uVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  uint *puVar4;
  bool bVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  uVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fb98;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar3 = FUN_0058a5b0(this,param_1);
  bVar5 = iVar3 != 0;
  FUN_004ece30(&param_1);
  local_4 = 0;
  do {
    FUN_004ece80(&param_1);
    bVar2 = FUN_0058a9e0(this,&param_1);
    if ((CONCAT31(extraout_var,bVar2) == 0) || (!bVar5)) {
      bVar5 = false;
    }
    else {
      bVar5 = true;
    }
    puVar4 = FUN_004ece40(&param_1);
    if (puVar4 != (uint *)0x0) {
      iVar3 = FUN_0058a2c0(this,&param_1,uVar1);
      if ((iVar3 == 0) || (!bVar5)) {
        bVar5 = false;
      }
      else {
        bVar5 = true;
      }
    }
    puVar4 = FUN_004ece40(&param_1);
  } while (puVar4 != (uint *)0x0);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar5;
}

