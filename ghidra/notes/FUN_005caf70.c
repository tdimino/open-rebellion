
void __thiscall
FUN_005caf70(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  int iVar1;
  
  iVar1 = *(int *)((int)this + 0x54);
  *(undefined4 *)(iVar1 + 0x3c) = param_1;
  *(undefined4 *)(iVar1 + 0x40) = param_2;
  *(undefined4 *)(iVar1 + 0x44) = param_3;
  *(undefined4 *)(iVar1 + 0x48) = param_4;
  return;
}

