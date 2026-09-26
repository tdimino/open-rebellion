
void __thiscall FUN_0045ccc0(void *this,void *param_1)

{
  void *pvVar1;
  uint *puVar2;
  void *pvVar3;
  int iVar4;
  uint uVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  pvVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630b88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar2 = FUN_004025b0(param_1,(uint *)&param_1);
  uVar5 = 0;
  local_4 = 0;
  pvVar3 = (void *)FUN_0060a860((void *)((int)this + 0x174),(ushort)*puVar2 | 0x100000);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (pvVar3 != (void *)0x0) {
    iVar4 = *(int *)(*(int *)((int)this + 0x194) + 0x9c);
    if (iVar4 == 1) {
      uVar5 = 1;
    }
    else if (iVar4 == 2) {
      uVar5 = 2;
    }
    iVar4 = FUN_004feea0(pvVar1,uVar5,3);
    if (iVar4 == 0) {
      iVar4 = *(int *)(*(int *)((int)this + 0x194) + 0x9c);
      if (iVar4 == 1) {
        uVar5 = 2;
      }
      else if (iVar4 == 2) {
        uVar5 = 1;
      }
      else {
        uVar5 = 0;
      }
      iVar4 = FUN_004feea0(pvVar1,uVar5,3);
      if (iVar4 == 0) {
        uVar5 = *(uint *)((int)pvVar1 + 0x24) >> 6 & 3;
        iVar4 = FUN_004feea0(pvVar1,uVar5,3);
      }
    }
    FUN_0045d140(this,pvVar3,uVar5,iVar4);
  }
  ExceptionList = local_c;
  return;
}

