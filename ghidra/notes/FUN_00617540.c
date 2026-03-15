
void * __thiscall FUN_00617540(void *this,int param_1)

{
  uint uVar1;
  byte bVar2;
  int iVar3;
  ushort local_14;
  undefined1 local_12;
  undefined1 local_11;
  undefined2 local_10;
  undefined1 local_e;
  undefined1 local_d;
  char local_c [12];
  
  local_10 = DAT_0066e5c8;
  local_e = DAT_0066e5ca;
  local_d = 0;
  local_14 = DAT_0066e578;
  local_12 = DAT_0066e57a;
  local_11 = 0;
  iVar3 = FUN_00615fc0((int *)this);
  if (iVar3 != 0) {
    if (param_1 != 0) {
                    /* WARNING: Load size is inaccurate */
      uVar1 = *(uint *)((int)this + *(int *)(*this + 4) + 0x24);
      if ((uVar1 & 0x60) == 0) {
        if ((0 < param_1) && ((uVar1 & 0x400) != 0)) {
          local_14 = CONCAT11(local_14._1_1_,0x2b);
        }
      }
      else {
        if ((uVar1 & 0x40) == 0) {
          local_10 = CONCAT11(0x6f,(char)local_10);
        }
        else {
          bVar2 = ~(byte)(uVar1 >> 4);
          local_10 = CONCAT11(bVar2,(char)local_10) & 0x20ff | 0x5800;
          local_14 = CONCAT11(bVar2,(undefined1)local_14) & 0x20ff | 0x5800;
        }
        if ((*(byte *)((int)this + *(int *)(*this + 4) + 0x24) & 0x80) != 0) {
          local_14 = CONCAT11(local_14._1_1_,0x30);
        }
      }
    }
    FUN_00618ed0(local_c,(char *)&local_10);
    FUN_006162b0(this,(char *)&local_14,local_c);
    FUN_00616040((int *)this);
  }
  return this;
}

