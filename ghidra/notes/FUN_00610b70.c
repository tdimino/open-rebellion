
void __thiscall FUN_00610b70(void *this,int param_1)

{
  uint uVar1;
  
  *(undefined ***)this = &PTR_LAB_00658518;
  *(undefined4 *)((int)this + 4) = *(undefined4 *)(param_1 + 4);
  *(undefined4 *)((int)this + 8) = *(undefined4 *)(param_1 + 8);
  *(undefined4 *)((int)this + 0xc) = *(undefined4 *)(param_1 + 0xc);
  uVar1 = (*(uint *)(param_1 + 0x10) ^ *(uint *)((int)this + 0x10)) & 7 ^
          *(uint *)((int)this + 0x10);
  *(uint *)((int)this + 0x10) = uVar1;
  uVar1 = (*(uint *)(param_1 + 0x10) ^ uVar1) & 8 ^ uVar1;
  *(uint *)((int)this + 0x10) = uVar1;
  uVar1 = (*(uint *)(param_1 + 0x10) ^ uVar1) & 0x10 ^ uVar1;
  *(uint *)((int)this + 0x10) = uVar1;
  uVar1 = (*(uint *)(param_1 + 0x10) ^ uVar1) & 0x40 ^ uVar1;
  *(uint *)((int)this + 0x10) = uVar1;
  *(uint *)((int)this + 0x10) = (*(uint *)(param_1 + 0x10) ^ uVar1) & 0x20 ^ uVar1;
  return;
}

