
int __thiscall FUN_0058a2c0(void *this,uint *param_1,int param_2)

{
  uint *puVar1;
  int iVar2;
  bool bVar3;
  int iVar4;
  int iVar5;
  undefined3 extraout_var;
  uint *puVar6;
  int iVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fc48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar4 = FUN_0058a3b0((int)this);
  puVar1 = param_1;
  iVar5 = FUN_00505190(param_1);
  if (iVar5 != 0) {
    FUN_004ece30(&param_1);
    iVar2 = param_2;
    local_4 = 0;
    do {
      FUN_004ece80(&param_1);
      bVar3 = FUN_0058ab00(this,(uint *)&param_1);
      if ((CONCAT31(extraout_var,bVar3) == 0) || (iVar4 == 0)) {
        iVar4 = 0;
      }
      else {
        iVar4 = 1;
      }
      puVar6 = FUN_004ece40((uint *)&param_1);
      if (puVar6 != (uint *)0x0) {
        iVar7 = FUN_0058a3f0(this,(uint *)&param_1,puVar1,iVar2);
        if ((iVar7 == 0) || (iVar4 == 0)) {
          iVar4 = 0;
        }
        else {
          iVar4 = 1;
        }
      }
      puVar6 = FUN_004ece40((uint *)&param_1);
      if (puVar6 == (uint *)0x0) break;
      iVar7 = FUN_004ece60((uint *)(iVar5 + 0x9c));
    } while (iVar7 == 0);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar4;
}

