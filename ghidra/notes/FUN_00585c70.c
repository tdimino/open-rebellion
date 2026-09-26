
uint __thiscall FUN_00585c70(void *this,int *param_1)

{
  int *piVar1;
  uint *puVar2;
  undefined4 *this_00;
  uint uVar3;
  int iVar4;
  int iVar5;
  bool bVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f5a8;
  local_c = ExceptionList;
  if (param_1 == (int *)0x0) {
    bVar6 = false;
    this_00 = (undefined4 *)0x0;
    ExceptionList = &local_c;
  }
  else {
    ExceptionList = &local_c;
    puVar2 = FUN_004025b0(param_1,(uint *)&param_1);
    local_4 = 0;
    this_00 = FUN_00585dc0(this,puVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar6 = this_00 != (undefined4 *)0x0;
  }
  uVar3 = 0;
  if ((bVar6) && (uVar3 = FUN_00584470(this_00,piVar1), uVar3 != 0)) {
    bVar6 = true;
    iVar5 = 0;
    do {
      iVar4 = FUN_005844e0(this_00,iVar5);
      if (iVar4 != 0) {
        bVar6 = false;
      }
      iVar5 = iVar5 + 1;
    } while (iVar5 < 3);
    if (bVar6) {
      uVar3 = FUN_00585d60(this,this_00);
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

