
void * __thiscall FUN_00429ce0(void *this,uint *param_1)

{
  int iVar1;
  undefined4 uVar2;
  uint *puVar3;
  void *pvVar4;
  uint *puVar5;
  uint uVar6;
  uint uVar7;
  uint uVar8;
  int *piVar9;
  uint *puStack_20;
  int *piStack_1c;
  uint uStack_18;
  uint uStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c45b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  PostMessageA(*(HWND *)((int)this + 0x18),0x409,0,0);
  puVar5 = param_1;
  piStack_1c = FUN_004f3970(*(int *)((int)this + 0x9c),param_1);
  if (piStack_1c == (int *)0x0) {
    ExceptionList = pvStack_c;
    return (void *)0x0;
  }
  uStack_18 = (ushort)((short)*puVar5 << 6) | 1;
  param_1 = (uint *)0x0;
  puStack_20 = (uint *)0x0;
  pvVar4 = (void *)FUN_00604500((void *)((int)this + 0x6c),uStack_18);
  if (pvVar4 != (void *)0x0) {
    ExceptionList = pvStack_c;
    return pvVar4;
  }
  FUN_0041d2d0(1);
  iVar1 = *(int *)((int)this + 0x9c);
  uVar6 = (-(uint)(iVar1 != 1) & 0x3c) + 0x3c;
  uVar7 = (-(uint)(iVar1 != 1) & 5) + 0x23;
  uStack_14 = (-(uint)(iVar1 != 1) & 0x41) + 300;
  puVar5 = (uint *)FUN_005f5060((int)this + 0x6c);
  puVar3 = puStack_20;
  for (; puStack_20 = puVar3, puVar5 != (uint *)0x0; puVar5 = (uint *)puVar5[4]) {
    if (((byte)puVar5[9] & 0x3f) == 1) {
      puStack_20 = puVar5;
      if (puVar5[10] == uVar6) {
        param_1 = puVar5;
        puStack_20 = puVar3;
      }
      if ((param_1 != (uint *)0x0) && (puStack_20 != (uint *)0x0)) break;
    }
    puVar3 = puStack_20;
  }
  puVar5 = puStack_20;
  if (param_1 == (uint *)0x0) {
    if (puStack_20 == (uint *)0x0) {
      FUN_00526560(piStack_1c,&param_1);
      if (DAT_00658bd8 / 2 <= (short)param_1) {
        uVar6 = uStack_14;
      }
      goto LAB_00429e6b;
    }
    if (puStack_20 != (uint *)0x0) goto LAB_00429e6b;
  }
  else {
    uVar6 = uStack_14;
    if (puStack_20 == (uint *)0x0) goto LAB_00429e6b;
  }
  FUN_00526560(piStack_1c,&puStack_20);
  if ((short)puStack_20 < DAT_00658bd8 / 2) {
    puVar5 = param_1;
  }
  uVar6 = puVar5[10];
  uVar7 = puVar5[0xb];
  FUN_005f54a0((void *)((int)this + 0x478),puVar5[9]);
  FUN_00600f90(this,puVar5[9]);
LAB_00429e6b:
  param_1 = (uint *)FUN_00618b70(0x1a4);
  uStack_4 = 0;
  if (param_1 == (uint *)0x0) {
    pvVar4 = (void *)0x0;
  }
  else {
    uVar2 = *(undefined4 *)((int)this + 0x1c);
    uVar8 = uStack_18;
    piVar9 = piStack_1c;
    puVar5 = (uint *)FUN_005ff440((int)this);
    pvVar4 = FUN_004591d0(param_1,uVar2,uVar6,uVar7,0xeb,0x168,(int)this,puVar5,uVar8,piVar9);
  }
  uStack_4 = 0xffffffff;
  if (pvVar4 == (void *)0x0) {
    ExceptionList = pvStack_c;
    return (void *)0x0;
  }
  FUN_005ffce0(pvVar4,0);
  FUN_0042ac70(this,(int)pvVar4);
  FUN_005f4f10((void *)((int)this + 0x6c),(int)pvVar4);
  SetWindowPos(*(HWND *)((int)pvVar4 + 0x18),(HWND)0x1,0,0,0,0,3);
  SetWindowPos(*(HWND *)(*(int *)((int)this + 0x248) + 0x18),*(HWND *)((int)pvVar4 + 0x18),0,0,0,0,3
              );
  ExceptionList = pvStack_c;
  return pvVar4;
}

