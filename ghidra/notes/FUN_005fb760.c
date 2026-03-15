
void * __thiscall FUN_005fb760(void *this,undefined4 *param_1,uint param_2)

{
  uint uVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655e28;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5590((undefined4 *)this);
  *(undefined ***)this = &PTR_FUN_0066de70;
  local_4 = 0;
  puVar2 = (undefined4 *)((int)this + 0x20);
  for (uVar1 = (param_2 & 0xffff) >> 2; uVar1 != 0; uVar1 = uVar1 - 1) {
    *puVar2 = *param_1;
    param_1 = param_1 + 1;
    puVar2 = puVar2 + 1;
  }
  for (uVar1 = param_2 & 3; uVar1 != 0; uVar1 = uVar1 - 1) {
    *(undefined1 *)puVar2 = *(undefined1 *)param_1;
    param_1 = (undefined4 *)((int)param_1 + 1);
    puVar2 = (undefined4 *)((int)puVar2 + 1);
  }
  FUN_005f5600(this,*(undefined4 *)((int)this + 0x20));
  ExceptionList = local_c;
  return this;
}

