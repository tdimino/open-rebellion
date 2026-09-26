
undefined4 __thiscall FUN_0055aa80(void *this,void *param_1)

{
  bool bVar1;
  void *pvVar2;
  int iVar3;
  undefined4 uVar4;
  bool bVar5;
  undefined1 local_14 [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649ac8;
  pvStack_c = ExceptionList;
  bVar5 = *(int *)((int)this + 0x58) == 4;
  ExceptionList = &pvStack_c;
  if ((((*(byte *)((int)this + 0x50) & 1) != 0) &&
      (ExceptionList = &pvStack_c, (*(byte *)((int)this + 0x60) & 1) == 0)) &&
     (ExceptionList = &pvStack_c, ((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) != 0xc0)) {
    ExceptionList = &pvStack_c;
    pvVar2 = FUN_0052e740(this,local_14);
    local_4 = 0;
    iVar3 = (**(code **)(*(int *)this + 0x214))(pvVar2);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar3 != 0) {
      bVar1 = true;
      goto LAB_0055ab14;
    }
  }
  bVar1 = false;
LAB_0055ab14:
  uVar4 = 1;
  if (bVar1) {
    if (bVar5) {
      uVar4 = FUN_0053aac0(this,param_1);
    }
  }
  else if (!bVar5) {
    uVar4 = FUN_0053ab10(this,param_1);
  }
  ExceptionList = pvStack_c;
  return uVar4;
}

