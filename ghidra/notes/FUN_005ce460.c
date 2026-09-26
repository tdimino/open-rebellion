
void * __thiscall
FUN_005ce460(void *this,undefined4 param_1,undefined1 param_2,undefined4 param_3,undefined4 param_4,
            uint *param_5)

{
  int iVar1;
  uint uVar2;
  uint uVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654448;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005a6210(this);
  *(undefined ***)this = &PTR_FUN_0066c928;
  local_4 = 0;
  uVar3 = *param_5;
  puVar4 = (undefined4 *)param_5[1];
  puVar5 = (undefined4 *)((int)this + 0x31);
  for (uVar2 = uVar3 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
    *puVar5 = *puVar4;
    puVar4 = puVar4 + 1;
    puVar5 = puVar5 + 1;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined1 *)puVar5 = *(undefined1 *)puVar4;
    puVar4 = (undefined4 *)((int)puVar4 + 1);
    puVar5 = (undefined4 *)((int)puVar5 + 1);
  }
  uVar3 = *param_5;
  *(undefined4 *)((int)this + 0x20) = param_1;
  *(uint *)((int)this + 0x1c) = uVar3 + 0x11;
  *(undefined1 *)((int)this + 0x28) = param_2;
  *(undefined4 *)((int)this + 0x29) = param_3;
  *(undefined4 *)((int)this + 0x2d) = param_4;
  iVar1 = FUN_005aaf90();
  FUN_005a6680(this,*(uint *)(iVar1 + 0x9ec));
  *(undefined4 *)((int)this + 0x24) = *(undefined4 *)((int)this + 0x18);
  ExceptionList = local_c;
  return this;
}

