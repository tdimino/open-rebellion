
bool __thiscall FUN_0050b890(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  
  bVar2 = true;
  if ((*(uint *)((int)this + 0x88) & 8) != 0) {
    if ((((byte)*(undefined4 *)((int)this + 0x24) & 0xc0) == 0xc0) ||
       ((*(uint *)((int)this + 0x88) & 1) == 0)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    iVar1 = FUN_0050a510(this,(uint)!bVar2,param_1);
    bVar2 = iVar1 != 0;
  }
  return bVar2;
}

