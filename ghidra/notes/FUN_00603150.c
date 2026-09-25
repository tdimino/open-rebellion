
void __thiscall FUN_00603150(void *this,undefined4 param_1,uint param_2)

{
  undefined4 *puVar1;
  bool bVar2;
  void *pvVar3;
  undefined3 extraout_var;
  undefined2 uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006562db;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (pvVar3 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_005fbd20(pvVar3,*(undefined4 *)((int)this + 0x1c),param_2,2);
  }
  local_4 = 0xffffffff;
  if ((pvVar3 != (void *)0x0) ||
     (bVar2 = FUN_005fc040((int *)0x0), CONCAT31(extraout_var,bVar2) != 0)) {
    uVar4 = (undefined2)param_2;
    switch(param_1) {
    case 0:
      puVar1 = *(undefined4 **)((int)this + 0x94);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
        if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
          InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
        }
      }
      *(void **)((int)this + 0x94) = pvVar3;
      *(undefined2 *)((int)this + 0xac) = uVar4;
      break;
    case 1:
      if (*(int *)((int)this + 0x98) != 0) {
        if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
          InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
        }
        puVar1 = *(undefined4 **)((int)this + 0x98);
        if (puVar1 != (undefined4 *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
      }
      *(void **)((int)this + 0x98) = pvVar3;
      *(undefined2 *)((int)this + 0xae) = uVar4;
      break;
    case 2:
    case 0x40:
      puVar1 = *(undefined4 **)((int)this + 0x9c);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
        if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
          InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
        }
      }
      *(void **)((int)this + 0x9c) = pvVar3;
      *(undefined2 *)((int)this + 0xb0) = uVar4;
      break;
    case 4:
      if (*(int *)((int)this + 0xa0) != 0) {
        if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
          InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
        }
        puVar1 = *(undefined4 **)((int)this + 0xa0);
        if (puVar1 != (undefined4 *)0x0) {
          FUN_005fbfa0(puVar1);
          FUN_00618b60((undefined *)puVar1);
        }
      }
      *(void **)((int)this + 0xa0) = pvVar3;
      *(undefined2 *)((int)this + 0xb2) = uVar4;
    }
  }
  ExceptionList = local_c;
  return;
}

