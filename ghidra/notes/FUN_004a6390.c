
UINT __thiscall FUN_004a6390(void *this,uint param_1,HPALETTE param_2,HWND param_3)

{
  HWND hWnd;
  int *piVar1;
  int iVar2;
  uint uVar3;
  int *piVar4;
  int *piStack_14;
  UINT local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637dd8;
  local_c = ExceptionList;
  hWnd = *(HWND *)(*(int *)((int)this + 0x148) + 0x18);
  ExceptionList = &local_c;
  piVar1 = FUN_004a25c0(this);
  if (piVar1 == (int *)0x0) {
    ExceptionList = local_c;
    return 0;
  }
  local_10 = 0;
  switch((uint)param_2 & 0xffff) {
  case 100:
    PostMessageA(hWnd,0x111,*(uint *)((int)this + 0x24) & 0xffff | 0x4660000,0);
    break;
  case 0x65:
    (**(code **)(*(int *)this + 0x30))();
    break;
  default:
    local_10 = FUN_004ac3a0(this,param_1,param_2,param_3);
    break;
  case 0x6a:
  case 0x6b:
    *(uint *)((int)this + 0x194) = (uint)param_2 >> 0x10;
    goto LAB_004a65ce;
  case 200:
    *(uint *)((int)this + 0x14c) = *(uint *)((int)this + 0x14c) | 1;
    if ((uint)param_2 >> 0x10 == 0x29a) {
      PostMessageA(*(HWND *)(*(int *)((int)this + 0x148) + 0x18),0x111,
                   *(uint *)((int)this + 0x24) & 0xffff | 0x29a0000,(LPARAM)param_3);
    }
    else if ((uint)param_2 >> 0x10 == 0x29b) {
      PostMessageA(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),0x467,*(WPARAM *)((int)this + 0x18),
                   0);
    }
    break;
  case 0xc9:
    uVar3 = (uint)param_2 >> 0x10;
    *(uint *)((int)this + 0x14c) = *(uint *)((int)this + 0x14c) & 0xfffffffe;
    if (uVar3 == 0x29a) {
      PostMessageA(*(HWND *)(*(int *)((int)this + 0x148) + 0x18),0x111,
                   *(uint *)((int)this + 0x24) & 0xffff | 0x29a0000,(LPARAM)param_3);
      ExceptionList = local_c;
      return local_10;
    }
    if (uVar3 != 0x29b) {
      if (uVar3 != 0x309) {
        ExceptionList = local_c;
        return 0;
      }
      piVar1 = (int *)(**(code **)(**(int **)((int)this + 0x188) + 8))();
      while (piStack_14 = piVar1, piStack_14 != (int *)0x0) {
        piVar1 = (int *)(**(code **)(*piStack_14 + 0xc))();
        if ((((*(byte *)(piStack_14 + 0x1a) & 4) != 0) && ((*(byte *)(piStack_14 + 0xf) & 1) != 0))
           && (piVar4 = FUN_004f36a0(*(int *)(*(int *)((int)this + 0x148) + 0x9c),
                                     (uint *)(piStack_14 + 0x1b)), piVar4 != (int *)0x0)) {
          if ((~piStack_14[0x1a] & 2U) == 0) {
            uVar3 = piStack_14[0x1a] & 0xfffffffd;
          }
          else {
            uVar3 = piStack_14[0x1a] | 2;
          }
          piStack_14[0x1a] = uVar3;
          FUN_004a3d40(this,(int)piVar4,(int *)&piStack_14);
          FUN_0060a280(*(void **)((int)this + 0x160));
        }
      }
      ExceptionList = local_c;
      return local_10;
    }
LAB_004a65ce:
    PostMessageA(*(HWND *)(*(int *)((int)this + 0x20) + 0x18),0x467,*(WPARAM *)((int)this + 0x18),0)
    ;
    FUN_004a5c00(this);
    break;
  case 0xca:
    if (*(int *)((int)this + 0x148) != 0) {
      FUN_004ece30(&param_3);
      local_4 = 0;
      iVar2 = FUN_004f6b70(piVar1,(uint *)&param_3);
      if (iVar2 != 0) {
        FUN_00429ce0(*(void **)((int)this + 0x148),(uint *)&param_3);
      }
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  ExceptionList = local_c;
  return local_10;
}

