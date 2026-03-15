
int FUN_0051b7f0(undefined4 *param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  int *this;
  uint uVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  uint uStack_18;
  uint uStack_14;
  undefined4 uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00642f68;
  local_c = ExceptionList;
  iVar6 = 1;
  ExceptionList = &local_c;
  this = (int *)FUN_00504d50();
  puVar2 = param_1;
  while (this != (int *)0x0) {
    uVar3 = FUN_00517760(this,puVar2);
    if ((uVar3 == 0) || (iVar6 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar6 = (**(code **)(*this + 0xc0))(puVar2);
    if ((iVar6 == 0) || (!bVar1)) {
      iVar6 = 0;
    }
    else {
      iVar6 = 1;
    }
    bVar1 = true;
    puStack_8 = (undefined1 *)0x1;
    uVar3 = (**(code **)(*this + 4))();
    if ((uVar3 < 0x30) || (uStack_18 <= uVar3)) {
      bVar1 = false;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      iVar5 = this[0x16];
      puVar4 = FUN_004025b0(this,(uint *)&param_1);
      uStack_4 = 2;
      iVar5 = FUN_00516360(puVar4,4,iVar5,puVar2);
      if ((iVar5 == 0) || (iVar6 == 0)) {
        iVar6 = 0;
      }
      else {
        iVar6 = 1;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
    uStack_14 = 1;
    uStack_10 = 0xff;
    uStack_4 = 0;
    this = (int *)FUN_004f6010(this,&uStack_14,1);
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return iVar6;
}

