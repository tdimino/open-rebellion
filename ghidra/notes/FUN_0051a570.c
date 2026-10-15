
bool __thiscall FUN_0051a570(void *this,uint *param_1,undefined4 param_2)

{
  uint uVar1;
  int iVar2;
  undefined4 uVar3;
  bool bVar4;
  
  bVar4 = true;
  uVar1 = (uint)(*(int *)((int)this + 0x158) == 0);
  *param_1 = uVar1;
  if (uVar1 == 0) {
    iVar2 = (**(code **)(**(int **)((int)this + 0x158) + 0x1cc))(param_2);
    bVar4 = iVar2 != 0;
    uVar3 = FUN_0051a5d0(*(void **)((int)this + 0x158));
    *(undefined4 *)((int)this + 0x158) = uVar3;
  }
  return bVar4;
}

