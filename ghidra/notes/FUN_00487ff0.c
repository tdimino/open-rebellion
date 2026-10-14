
void __thiscall FUN_00487ff0(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)((int)this + 0x58);
  if ((iVar1 != 0) && (param_1 == 0)) {
    FUN_00487eb0(this,iVar1);
    *(undefined4 *)((int)this + 0x58) = 0;
    return;
  }
  if ((iVar1 == 0) && (param_1 != 0)) {
    *(undefined4 *)((int)this + 0x58) = *(undefined4 *)((int)this + 0x54);
    FUN_00487eb0(this,1);
  }
  return;
}

