
int * __thiscall FUN_004774c0(void *this,undefined4 *param_1)

{
  int *piVar1;
  void *local_4;
  
  piVar1 = (int *)0x0;
  local_4 = this;
  if (*(int *)((int)this + 0x68) != 0) {
    FUN_00477810(this);
    FUN_00477950(this);
    *(undefined4 *)((int)this + 0x68) = 0;
  }
  *param_1 = 0;
  if (*(void **)((int)this + 0x5c) == (void *)0x0) {
    *param_1 = 1;
  }
  else {
    switch(*(undefined4 *)((int)this + 0x1c)) {
    case 1:
      FUN_004fcdd0((undefined4 *)((int)this + 0x60));
      FUN_004fcdd0((undefined4 *)((int)this + 100));
      piVar1 = FUN_00477a40(this);
      *(undefined4 *)((int)this + 0x1c) = 2;
      break;
    case 2:
      FUN_00477cb0((int)this);
      FUN_004adaf0(*(void **)((int)this + 0x5c),(int *)((int)this + 0x6c));
      *(undefined4 *)((int)this + 0x1c) = 3;
      break;
    case 3:
      piVar1 = (int *)FUN_00477e80(this,&local_4);
      if (local_4 != (void *)0x0) {
        *(undefined4 *)((int)this + 0x1c) = 4;
      }
      break;
    case 4:
      FUN_00478000((int)this);
      FUN_00478140(this);
      *(undefined4 *)((int)this + 0x1c) = 5;
      break;
    case 5:
      piVar1 = FUN_004adb70(*(void **)((int)this + 0x5c),&local_4);
      if (local_4 != (void *)0x0) {
        *(undefined4 *)((int)this + 0x1c) = 6;
      }
      break;
    case 6:
      FUN_00479560((int)this);
      FUN_004799d0((int)this);
      (**(code **)(**(int **)((int)this + 0x5c) + 0x20))((int)this + 0x60,(int)this + 100);
      FUN_00479a00((int)this);
      *(undefined4 *)((int)this + 0x1c) = 0;
      *param_1 = 1;
    default:
      *(undefined4 *)((int)this + 0x1c) = 1;
      *(undefined4 *)((int)this + 0x68) = 1;
    }
  }
  if (piVar1 != (int *)0x0) {
    *(undefined4 *)((int)this + 0x68) = 1;
  }
  return piVar1;
}

