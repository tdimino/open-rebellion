
void __thiscall FUN_00499de0(int param_1,int param_2,uint *param_3,int param_4)

{
  bool bVar1;
  bool bVar2;
  int *piVar3;
  uint *puVar4;
  int *piVar5;
  uint uVar6;
  void *this;
  undefined4 *puVar7;
  uint auStack_28 [7];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006369d8;
  pvStack_c = ExceptionList;
  if (param_2 != 0x106) {
    return;
  }
  ExceptionList = &pvStack_c;
  piVar3 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
  this = (void *)0x0;
  bVar2 = false;
  *(undefined4 *)(param_1 + 0x40) = 0;
  if ((param_4 != 0) || (piVar3 == (int *)0x0)) goto LAB_00499f9b;
  auStack_28[1] = 0x14;
  auStack_28[2] = 0x1c;
  if ((*param_3 >> 0x18 < 0x14) || (0x1b < *param_3 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar3 = FUN_004f2e20(*(int *)(param_1 + 0x20),param_3);
    if (piVar3 == (int *)0x0) {
LAB_00499f4e:
      *(undefined4 *)(param_1 + 0x34) = 0x80;
    }
    else {
      this = (void *)piVar3[7];
      *(undefined4 *)(param_1 + 0x34) = 0x80;
    }
  }
  else {
    auStack_28[3] = 0x1c;
    auStack_28[4] = 0x20;
    if ((*param_3 >> 0x18 < 0x1c) || (0x1f < *param_3 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      bVar2 = true;
      piVar3 = FUN_004f3590(*(int *)(param_1 + 0x20),param_3);
      this = (void *)piVar3[7];
      puVar4 = FUN_004025b0(this,auStack_28);
      auStack_28[5] = 0x14;
      auStack_28[6] = 0x1c;
      uStack_4 = 0;
      if ((*puVar4 >> 0x18 < 0x14) || (0x1b < *puVar4 >> 0x18)) {
        bVar1 = false;
      }
      FUN_00619730();
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        this = *(void **)((int)this + 0x1c);
        goto LAB_00499f4e;
      }
      *(undefined4 *)(param_1 + 0x34) = 0x40;
    }
  }
  if ((piVar3 != (int *)0x0) && (this != (void *)0x0)) {
    *(undefined4 *)(param_1 + 0x40) = 1;
  }
  if (*(int *)(param_1 + 0x20) == 1) {
    if (((byte)piVar3[9] & 0xc0) != 0x80) goto LAB_00499f9b;
  }
  else if ((*(int *)(param_1 + 0x20) != 2) || (((byte)piVar3[9] & 0xc0) != 0x40)) goto LAB_00499f9b;
  *(undefined4 *)(param_1 + 0x40) = 0;
LAB_00499f9b:
  if (*(int *)(param_1 + 0x40) != 0) {
    FUN_004f26d0((void *)(param_1 + 0x5c),param_3);
    if (bVar2) {
      *(undefined4 *)(param_1 + 0x28) = 9;
      *(ushort *)(param_1 + 0x30) = (*(int *)(param_1 + 0x20) != 1) + 0x457;
      if (this == (void *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        puVar7 = (undefined4 *)((int)this + 0x30);
      }
      if (piVar3 == (int *)0x0) {
        piVar5 = (int *)0x0;
      }
      else {
        piVar5 = piVar3 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7040,piVar5,puVar7,(undefined4 *)0x0,(undefined4 *)0x0
                  );
      if (this == (void *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        puVar7 = (undefined4 *)((int)this + 0x30);
      }
      if (piVar3 == (int *)0x0) {
        piVar5 = (int *)0x0;
      }
      else {
        piVar5 = piVar3 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7042,piVar5,puVar7,(undefined4 *)0x0,(undefined4 *)0x0
                  );
    }
    else {
      *(undefined4 *)(param_1 + 0x28) = 8;
      if (this == (void *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        puVar7 = (undefined4 *)((int)this + 0x30);
      }
      if (piVar3 == (int *)0x0) {
        piVar5 = (int *)0x0;
      }
      else {
        piVar5 = piVar3 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7041,piVar5,puVar7,(undefined4 *)0x0,(undefined4 *)0x0
                  );
      if (this == (void *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        puVar7 = (undefined4 *)((int)this + 0x30);
      }
      if (piVar3 == (int *)0x0) {
        piVar5 = (int *)0x0;
      }
      else {
        piVar5 = piVar3 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7043,piVar5,puVar7,(undefined4 *)0x0,(undefined4 *)0x0
                  );
      *(undefined2 *)(param_1 + 0x30) = 0x45e;
    }
    uVar6 = FUN_004c5000(piVar3);
    *(short *)(param_1 + 0x2e) = (short)uVar6;
  }
  ExceptionList = pvStack_c;
  return;
}

