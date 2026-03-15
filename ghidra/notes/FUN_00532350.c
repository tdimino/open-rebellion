
void __thiscall FUN_00532350(void *this,int param_1,void *param_2)

{
  int *this_00;
  int iVar1;
  
  iVar1 = FUN_004fb810(this,param_1,param_2);
  if ((iVar1 != 0) && (param_1 != 0)) {
    FUN_00540230((void *)((int)this + 0xcc),DAT_006bb50c);
    this_00 = (int *)((int)this + 0xd4);
    FUN_00540230(this_00,DAT_006bb510);
    FUN_00540270(this_00,DAT_006bb508);
    FUN_0053fa60(0x381,1,this,this_00,param_2);
  }
  return;
}

