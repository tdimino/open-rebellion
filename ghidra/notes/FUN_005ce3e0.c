
void * __thiscall FUN_005ce3e0(void *this,uint *param_1)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654428;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005a6210((undefined4 *)this);
  *(undefined ***)this = &PTR_FUN_0066c928;
  local_4 = 0;
  uVar2 = *param_1;
  puVar3 = (undefined4 *)param_1[1];
  puVar4 = (undefined4 *)((int)this + 0x20);
  for (uVar1 = uVar2 >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar4 = *puVar3;
    puVar3 = puVar3 + 1;
    puVar4 = puVar4 + 1;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *(undefined1 *)puVar4 = *(undefined1 *)puVar3;
    puVar3 = (undefined4 *)((int)puVar3 + 1);
    puVar4 = (undefined4 *)((int)puVar4 + 1);
  }
  FUN_005f5600(this,*(undefined4 *)((int)this + 0x24));
  *(uint *)((int)this + 0x1c) = *param_1;
  ExceptionList = local_c;
  return this;
}

