
void * __thiscall FUN_00498e90(void *this,char *param_1,int param_2,int param_3)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636798;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4b50(this,param_2);
  local_4 = 0;
  *(undefined ***)this = &PTR_FUN_0065ba20;
  if (param_3 == 0) {
    iVar3 = *(int *)((int)this + 0x20);
  }
  else {
    iVar3 = 2 - (uint)(param_2 != 1);
  }
  uVar1 = FUN_00526bd0();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  *(undefined4 *)((int)this + 0x40) = 1;
  if (iVar3 == 1) {
    *(undefined2 *)((int)this + 0x2e) = 0x410;
    *(undefined2 *)((int)this + 0x30) = 0x461;
  }
  else {
    *(undefined2 *)((int)this + 0x2e) = 0x411;
    *(undefined2 *)((int)this + 0x30) = 0x462;
  }
  FUN_00499010(this,param_1);
  iVar3 = FUN_004f3dd0(*(int *)((int)this + 0x20),2 - (*(int *)((int)this + 0x20) != 1));
  if (param_3 == 0) {
    if (iVar3 == 0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      puVar2 = (undefined4 *)(iVar3 + 0x30);
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x71f1,puVar2,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
  }
  else {
    if (iVar3 == 0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      puVar2 = (undefined4 *)(iVar3 + 0x30);
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x71f0,puVar2,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    *(undefined4 *)((int)this + 0x28) = 0x29;
  }
  ExceptionList = local_c;
  return this;
}

