
void * __thiscall FUN_0048b2e0(void *this,undefined4 param_1,int param_2)

{
  undefined4 uVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006350f8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_1);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065b188;
  uVar1 = FUN_0048b450();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x28) = 0x2a;
  *(undefined4 *)((int)this + 0x40) = 1;
  iVar2 = *(int *)(param_2 + 0x1c) >> 4;
  if (*(int *)((int)this + 0x20) == 1) {
    *(undefined2 *)((int)this + 0x2e) = 0x42f;
    *(undefined2 *)((int)this + 0x30) = 0x461;
    uVar3 = iVar2 * 3 + 0x6001;
    uVar4 = iVar2 * 3 + 0x6002;
  }
  else {
    *(undefined2 *)((int)this + 0x2e) = 0x430;
    *(undefined2 *)((int)this + 0x30) = 0x462;
    uVar4 = iVar2 * 3 + 0x6802;
    uVar3 = (iVar2 + 0x22ab) * 3;
  }
  FUN_0060b9d0((void *)((int)this + 0x44),uVar3,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0,(undefined4 *)0x0);
  FUN_0060b9d0((void *)((int)this + 0x50),uVar4,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0,(undefined4 *)0x0);
  ExceptionList = local_c;
  return this;
}

