
void __thiscall
FUN_004843f0(void *this,void *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4)

{
  int iVar1;
  int iVar2;
  
  FUN_0058f0c0(this,param_1);
  *(undefined4 *)((int)this + 0x38) = param_2;
  *(undefined4 *)((int)this + 0x3c) = param_3;
  *(undefined4 *)((int)this + 0x40) = param_4;
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4990((void *)((int)this + 0x24),param_1);
  FUN_004ece90((void *)((int)this + 0x28),param_1);
  FUN_004ece90((void *)((int)this + 0x2c),param_1);
  FUN_005f4d90(param_1,(int)this + 0x30);
  FUN_005f4d90(param_1,(int)this + 0x34);
  FUN_005f4d90(param_1,(undefined4 *)((int)this + 0x1c));
  iVar2 = (int)this + 0x44;
  iVar1 = 0x1e;
  do {
    FUN_005f4d90(param_1,iVar2);
    iVar2 = iVar2 + 4;
    iVar1 = iVar1 + -1;
  } while (iVar1 != 0);
  *(undefined4 *)((int)this + 0x1c) = 1;
  return;
}

