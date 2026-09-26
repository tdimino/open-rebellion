
void * __thiscall
FUN_0040d7a0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,undefined4 param_7,undefined4 param_8,int param_9,
            undefined4 param_10,uint param_11,undefined4 param_12,undefined4 param_13,
            undefined4 param_14)

{
  DWORD DVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a2b8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00606380(this,param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10
               ,param_11,param_12,0,0);
  *(undefined ***)this = &PTR_FUN_00658788;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x50) = 0x44000000;
  DVar1 = timeGetTime();
  *(DWORD *)((int)this + 0x134) = DVar1;
  *(undefined4 *)((int)this + 0x138) = 0;
  iVar2 = FUN_00597480();
  if (iVar2 == 0) {
    *(undefined4 *)((int)this + 0x11c) = 1;
  }
  else {
    uVar3 = FUN_005974a0();
    *(undefined4 *)((int)this + 0x11c) = uVar3;
  }
  FUN_00401980(*(undefined4 *)((int)this + 0x11c));
  puVar4 = &DAT_006b1224;
  do {
    if ((int)puVar4 < 0x6b1239) {
      *puVar4 = 0;
    }
    puVar4 = puVar4 + 1;
  } while ((int)puVar4 < 0x6b123c);
  DAT_006b1248 = timeGetTime();
  *(undefined4 *)((int)this + 0x114) = 0;
  *(undefined4 *)((int)this + 0x118) = 0;
  *(undefined4 *)((int)this + 0x120) = param_13;
  *(undefined4 *)((int)this + 0x130) = 0;
  *(undefined4 *)((int)this + 0x170) = 0;
  *(undefined4 *)((int)this + 0x174) = 0;
  *(undefined4 *)((int)this + 0x178) = 0;
  *(undefined4 *)((int)this + 0x17c) = 0;
  *(undefined4 *)((int)this + 0x180) = 0;
  *(undefined4 *)((int)this + 0x184) = 0;
  *(undefined4 *)((int)this + 0x188) = 0;
  *(undefined4 *)((int)this + 0x18c) = 0;
  *(undefined4 *)((int)this + 400) = 0;
  *(undefined4 *)((int)this + 0x13c) = 0;
  *(undefined4 *)((int)this + 0x140) = 0;
  *(undefined4 *)((int)this + 0x144) = 0;
  *(undefined4 *)((int)this + 0x148) = 0;
  *(undefined4 *)((int)this + 0x14c) = 0;
  *(undefined4 *)((int)this + 0x150) = 0;
  *(undefined4 *)((int)this + 0x154) = 0;
  *(undefined4 *)((int)this + 0x158) = 0;
  *(undefined4 *)((int)this + 0x15c) = 0;
  *(undefined4 *)((int)this + 0x160) = 0;
  *(undefined4 *)((int)this + 0x164) = 0;
  *(undefined4 *)((int)this + 0x168) = 0;
  *(undefined4 *)((int)this + 0x16c) = 0;
  *(undefined4 *)((int)this + 0x124) = param_14;
  *(undefined4 *)((int)this + 0x128) = 1;
  *(undefined4 *)((int)this + 300) = 0x2774;
  DAT_006b1208 = 1;
  ExceptionList = local_c;
  return this;
}

