
void __thiscall FUN_00418db0(void *this,uint *param_1)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  uint uVar5;
  uint local_64;
  uint local_60 [21];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062b160;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_64);
  bVar2 = false;
  local_4 = 0;
  FUN_004f26d0(&local_64,param_1);
  do {
    local_60[1] = 0x90;
    uVar5 = local_64 >> 0x18;
    local_60[2] = 0x98;
    if ((uVar5 < 0x90) || (0x97 < uVar5)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      FUN_00431170((void *)((int)this + 0x2c),&local_64);
LAB_00418e35:
      bVar2 = true;
    }
    else {
      local_60[3] = 0x14;
      local_60[4] = 0x1c;
      if ((uVar5 < 0x14) || (0x1b < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_00430860((void *)((int)this + 0x58),&local_64,(int *)param_1);
        goto LAB_00418e35;
      }
      local_60[5] = 8;
      local_60[6] = 0x10;
      if ((uVar5 < 8) || (0xf < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        if (*param_1 != local_64) {
          FUN_0042ff70((void *)((int)this + 0x78),&local_64);
        }
        goto LAB_00418e35;
      }
      local_60[7] = 0x30;
      local_60[8] = 0x40;
      if ((uVar5 < 0x30) || (0x3f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_00403bf0((void *)((int)this + 0x8c),&local_64);
        goto LAB_00418e35;
      }
      local_60[9] = 0x40;
      local_60[10] = 0x80;
      if ((uVar5 < 0x40) || (0x7f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_0042f7d0((void *)((int)this + 0xa8),(int *)&local_64);
        goto LAB_00418e35;
      }
      local_60[0xb] = 0x80;
      local_60[0xc] = 0x90;
      if ((uVar5 < 0x80) || (0x8f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_00430c90((void *)((int)this + 0x44),&local_64);
        goto LAB_00418e35;
      }
      local_60[0xd] = 0xf1;
      local_60[0xe] = 0xf2;
      if ((uVar5 < 0xf1) || (0xf1 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_00418e35;
      local_60[0xf] = 0xf2;
      local_60[0x10] = 0xf3;
      if ((uVar5 < 0xf2) || (0xf2 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_00418e35;
      local_60[0x11] = 0xf8;
      local_60[0x12] = 0xf9;
      if ((uVar5 < 0xf8) || (0xf8 < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_00418e35;
      local_60[0x13] = 0x98;
      local_60[0x14] = 0xa0;
      if ((uVar5 < 0x98) || (0x9f < uVar5)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_00418e35;
      piVar3 = FUN_004f2d10(*(int *)this,&local_64);
      if ((piVar3 == (int *)0x0) || ((void *)piVar3[7] == (void *)0x0)) goto LAB_00418e35;
      puVar4 = FUN_004025b0((void *)piVar3[7],local_60);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_64,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
    }
    if (bVar2) {
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    }
  } while( true );
}

