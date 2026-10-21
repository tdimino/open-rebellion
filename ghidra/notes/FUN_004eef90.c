
void __thiscall FUN_004eef90(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int local_4;
  
  local_4 = 0;
  bVar1 = FUN_004f6b50(this,&local_4);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    if (((((*(uint *)((int)this + 0x24) ^ *(uint *)(local_4 + 0x24)) & 0xc0) != 0) ||
        (iVar2 = 1, (*(byte *)((int)this + 0xac) & 1) != 0)) ||
       ((*(byte *)((int)this + 0x50) & 8) != 0)) {
      iVar2 = 0;
    }
    FUN_004ee870(this,iVar2,param_1);
  }
  return;
}

